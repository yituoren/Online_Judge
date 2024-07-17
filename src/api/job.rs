use actix_web::web::Buf;
use actix_web::{delete, put, HttpResponse};
use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use clap::builder::Str;
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use tokio::sync::mpsc;
use tokio::time;
use std::process::{Command, ExitStatus, Stdio};
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{read_to_string, BufRead, BufReader, Error, ErrorKind, Result, Write};
use chrono::Utc;
use std::cmp::min;
use std::time::Duration;
use wait_timeout::ChildExt;

use crate::globals::{JOB_LIST, USER_LIST};
use crate::arg::{Config, Language, Problem};
use crate::api::error::HttpError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostJob
{
    pub source_code: String,
    pub language: String,
    pub user_id: usize,
    pub contest_id: usize,
    pub problem_id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job
{
    pub id: usize,
    pub created_time: String,
    pub updated_time: String,
    pub submission: PostJob,
    pub state: String,
    pub result: String,
    pub score: f64,
    pub cases: Vec<JobCase>,
}

impl Job
{
    pub fn new(id: usize, post_job: PostJob, case_num: usize) -> Job
    {
        let mut cases: Vec<JobCase> = Vec::new();
        for count in 0..=case_num
        {
            cases.push(JobCase {
                id: count,
                result: "Waiting".to_string(),
                time: 0,
                memory: 0,
                info: String::new(),
            })
        }
        Job {
            id,
            created_time: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            updated_time: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            submission: post_job.clone(),
            state: "Queueing".to_string(),
            result: "Waiting".to_string(),
            score: 0.0,
            cases,
        }
    }

    pub fn from(mut old_job: Job) -> Job
    {
        for case in old_job.cases.iter_mut()
        {
            case.result = "Waiting".to_string();
            case.time = 0;
            case.memory = 0;
            case.info = String::new();
        }
        old_job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        old_job.state = "Queueing".to_string();
        old_job.result = "Waiting".to_string();
        old_job.score = 0.0;
        old_job
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobCase
{
    pub id: usize,
    pub result: String,
    pub time: u64,
    pub memory: u64,
    pub info: String,
}

#[derive(Deserialize)]
pub struct JobQuery
{
    user_id: Option<usize>,
    user_name: Option<String>,
    contest_id: Option<usize>,
    problem_id: Option<usize>,
    language: Option<String>,
    from: Option<String>,
    to: Option<String>,
    state: Option<String>,
    result: Option<String>,
}

#[get("/jobs/{jobid}")]
pub async fn get_jobs_id(get_job: web::Path<usize>) -> HttpResponse
{
    let id = get_job;
    let lock = JOB_LIST.lock().await;
    if *id >= lock.len()
    {
        return HttpResponse::NotFound()
            .content_type("application/json")
            .json(HttpError {
                code: 3,
                reason: "ERR_NOT_FOUND".to_string(),
                message: "Job ".to_string() + &id.to_string() + " not found."
            });
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(lock[*id].clone())
}

#[get("/jobs")]
pub async fn get_jobs_query(query: web::Query<JobQuery>) -> HttpResponse
{
    let lock = JOB_LIST.lock().await;
    let mut job_list = lock.clone();
    drop(lock);

    if let Some(user_id) = query.user_id
    {
        job_list.retain(|x| x.submission.user_id == user_id);
    }
    if let Some(problem_id) = query.problem_id
    {
        job_list.retain(|x| x.submission.problem_id == problem_id);
    }
    if let Some(contest_id) = query.contest_id
    {
        job_list.retain(|x| x.submission.contest_id == contest_id);
    }
    if let Some(language) = &query.language
    {
        job_list.retain(|x| x.submission.language == *language);
    }
    if let Some(from) = &query.from
    {
        job_list.retain(|x| x.created_time >= *from);
    }
    if let Some(to) = &query.to
    {
        job_list.retain(|x| x.created_time <= *to);
    }
    if let Some(state) = &query.state
    {
        job_list.retain(|x| x.state >= *state);
    }
    if let Some(result) = &query.result
    {
        job_list.retain(|x| x.result >= *result);
    }
    if let Some(user_name) = &query.user_name
    {
        match USER_LIST.lock().await.iter().position(|x| x.name == *user_name)
        {
            Some(id) => job_list.retain(|y| y.submission.user_id == id),
            None => job_list.clear(),
        }
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(job_list)
}

#[put("/jobs/{jobid}")]
pub async fn put_jobs_id(put_job: web::Path<usize>) -> HttpResponse
{
    let id = *put_job;
    let mut lock = JOB_LIST.lock().await;
    if id >= lock.len()
    {
        return HttpResponse::NotFound()
            .content_type("application/json")
            .json(HttpError {
                code: 3,
                reason: "ERR_NOT_FOUND".to_string(),
                message: "Job ".to_string() + &id.to_string() + " not found."
            });
    }
    if lock[id].state != "Finished"
    {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(HttpError {
                code: 2,
                reason: "ERR_INVALID_STATE".to_string(),
                message: "Job ".to_string() + &id.to_string() + " not finished."
            });
    }
    lock[id] = Job::from(lock[id].clone());
    HttpResponse::Ok()
        .content_type("application/json")
        .json(lock[id].clone())
}

#[delete("/jobs/{jobid}")]
pub async fn delete_jobs(delete_job: web::Path<usize>) -> HttpResponse
{
    let id = *delete_job;
    let mut lock = JOB_LIST.lock().await;
    if id >= lock.len()
    {
        return HttpResponse::NotFound()
        .content_type("application")
        .json(HttpError {
            code: 3,
            reason: "ERR_NOT_FOUND".to_string(),
            message: "Job ".to_string() + &id.to_string() + " not found."
        });
    }
    if lock[id].state == "Queueing"
    {
        lock.remove(id);
    }
    else
    {
        return HttpResponse::BadRequest()
        .content_type("application")
        .json(HttpError {
            code: 2,
            reason: "ERR_NOT_FOUND".to_string(),
            message: "Job ".to_string() + &id.to_string() + " not queueing."
        });
    }
    HttpResponse::Ok().into()
}

#[post("/jobs")]
pub async fn post_jobs(post_job: web::Json<PostJob>, config: web::Data<Config>) -> HttpResponse
{
    log::info!("Post job: {:?}", post_job);

    let lock = USER_LIST.lock().await;
    if post_job.user_id >= lock.len()
    {
        return HttpResponse::NotFound()
            .content_type("application")
            .json(HttpError {
                code: 3,
                reason: "ERR_NOT_FOUND".to_string(),
                message: "User ".to_string() + &post_job.user_id.to_string() + " not found."
            })
    }
    drop(lock);

    if !config.languages.iter().any(|x| x.name == post_job.language)
    {
        return HttpResponse::NotFound()
        .content_type("application")
        .json(HttpError {
            code: 3,
            reason: "ERR_NOT_FOUND".to_string(),
            message: "Language ".to_string() + &post_job.language + " not found."
        })
    }

    let mut problem = Problem {
        id: 0,
        name: String::new(),
        misc: serde_json::Value::Object(Map::new()),
        problem_type: String::new(),
        cases: Vec::new(),
    };
    if let Ok(tmp) = find_problem(&config.problems, post_job.problem_id)
    {
        problem = tmp
    }
    else
    {
        return HttpResponse::NotFound()
            .content_type("application/json")
            .json(HttpError {
                code: 3,
                reason: "ERR_NOT_FOUND".to_string(),
                message: "Problem ".to_string() + &post_job.problem_id.to_string() + " not found."
            });
    }

    let mut lock = JOB_LIST.lock().await;
    let id = lock.len();
    let job = Job::new(id, post_job.clone(), problem.cases.len());
    lock.push(job.clone());
    drop(lock);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(job)
}

pub async fn job_producer(mut tx: mpsc::Sender<Job>, config: Config)
{
    loop
    {
        let lock = JOB_LIST.lock().await;
        let job_list = lock.clone();
        drop(lock);
        for mut job in job_list.into_iter()
        {
            if job.state != "Queueing"
            {
                continue;
            }
            job.state = "Running".to_string();
            job.result = "Running".to_string();
            job.cases[0].result = "Running".to_string();
            job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            tx.send(job.clone()).await.unwrap();
            
            let _ = create_dir_all("./".to_string() + &job.id.to_string());
            let path = "./".to_string() + &job.id.to_string() + "/";
            let problem = find_problem(&config.problems, job.submission.problem_id).unwrap();

            match compile_program(&path, &job.submission, &config.languages).await
            {
                Ok(status) =>
                {
                    if status.success()
                    {
                        job.cases[0].result = "Compilation Success".to_string();
                        job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                        tx.send(job.clone()).await.unwrap();
                    }
                    else
                    {
                        job.result = "Compilation Error".to_string();
                        job.cases[0].result = "Compilation Error".to_string();
                        job.state = "Finished".to_string();
                        job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                        tx.send(job.clone()).await.unwrap();
                        continue;
                    }
                }
                Err(_) =>
                {
                    job.result = "Compilation Error".to_string();
                    job.cases[0].result = "Compilation Error".to_string();
                    job.state = "Finished".to_string();
                    job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                    tx.send(job.clone()).await.unwrap();
                    continue;
                }
            }

            let mut count: usize = 1;
            for case in problem.cases.iter()
            {
                job.cases[count].result = "Running".to_string();
                job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                tx.send(job.clone()).await.unwrap();
                let out_file = File::create(path.clone() + &count.to_string() + ".out").unwrap();
                let time_limit = Duration::from_micros(problem.cases[count - 1].time_limit);
                let start = Utc::now();
                match run_case(&path, File::open(case.input_file.clone()).unwrap(), out_file, time_limit).await
                {
                    Ok(Some(status)) =>
                    {
                        if status.success()
                        {
                            let end = Utc::now();
                            let duration: u64 = (end - start).num_microseconds().unwrap() as u64;
                            let output_file = File::open(path.clone() + &count.to_string() + ".out").unwrap();
                            let output = BufReader::new(output_file);
                            let mut output_lines: Vec<String> = Vec::new();
                            let answer_file = File::open(case.answer_file.clone()).unwrap();
                            let answer = BufReader::new(answer_file);
                            let mut answer_lines: Vec<String> = Vec::new();
                            let mut same: bool = true;
                            if problem.problem_type == "standard"
                            {
                                for line in output.lines()
                                {
                                    output_lines.push(line.unwrap().trim_end().to_string());
                                }
                                for line in answer.lines()
                                {
                                    answer_lines.push(line.unwrap().trim_end().to_string());
                                }
                                for i in 0..min(output_lines.len(), answer_lines.len())
                                {
                                    if output_lines[i] != answer_lines[i]
                                    {
                                        same = false;
                                        break;
                                    }
                                }
                            }
                            else
                            {
                                same = read_to_string(output).unwrap() == read_to_string(answer).unwrap();
                            }
                            if same
                            {
                                job.cases[count].result = "Accepted".to_string();
                                job.cases[count].time = duration;
                                job.score += problem.cases[count - 1].score;
                                job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                                tx.send(job.clone()).await.unwrap();
                            }
                            else
                            {
                                job.cases[count].result = "Wrong Answer".to_string();
                                job.cases[count].time = duration;
                                if job.result == "Running" { job.result = "Wrong Answer".to_string(); }
                                job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                                tx.send(job.clone()).await.unwrap();
                            }
                        }
                        else
                        {
                            job.cases[count].result = "Runtime Error".to_string();
                            if job.result == "Running" { job.result = "Runtime Error".to_string(); }
                            job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                            tx.send(job.clone()).await.unwrap();
                        }
                    }
                    Ok(None) =>
                    {
                        job.cases[count].result = "Time Limit Exceeded".to_string();
                        if job.result == "Running" { job.result = "Time Limit Exceeded".to_string(); }
                        job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                        tx.send(job.clone()).await.unwrap();
                    }
                    Err(_) =>
                    {
                        job.cases[count].result = "Runtime Error".to_string();
                        job.result = "Runtime Error".to_string();
                        job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
                        tx.send(job.clone()).await.unwrap();
                    }
                }
                count += 1;
            }
            if job.result == "Running"
            {
                job.result = "Accepted".to_string();
            }
            job.state = "Finished".to_string();
            job.updated_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            tx.send(job.clone()).await.unwrap();
    
            let _ = remove_dir_all("./".to_string() + &job.id.to_string());
        }
        time::sleep(time::Duration::from_millis(500)).await;
    }
}

pub async fn job_consumer(mut rx: mpsc::Receiver<Job>)
{
    while let Some(job) = rx.recv().await
    {
        let mut lock = JOB_LIST.lock().await;
        lock[job.id] = job.clone();
        drop(lock);
    }
}

async fn run_case(path: &str, in_file: File, out_file: File, time_limit: Duration) -> Result<Option<ExitStatus>>
{
    let mut child = Command::new(path.to_string() + "main")
                    .stdin(Stdio::from(in_file))
                    .stdout(Stdio::from(out_file))
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap();
    let result = child.wait_timeout(time_limit);
    if let Ok(None) = result
    {
        child.kill().unwrap();
    }
    result
}

async fn compile_program(path: &str, job: &PostJob, languages: &Vec<Language>) -> Result<ExitStatus>
{
    if let Some(language) = languages.iter().find(|&x| x.name == job.language)
    {
        let mut args = language.command.clone();
        let command = args.remove(0);
        for arg in args.iter_mut()
        {
            if arg == "%INPUT%"
            {
                *arg = path.to_string() + &language.file_name;
            }
            else if arg == "%OUTPUT%"
            {
                *arg = path.to_string() + "main";
            }
        }
        let mut src = File::create(path.to_string() + &language.file_name).unwrap();
        let _ = src.write(job.source_code.as_bytes());
        Command::new(command).args(args).status()
    }
    else
    {
        Err(Error::new(ErrorKind::Unsupported, "No Such Language"))
    }
}

fn find_problem(problems: &Vec<Problem>, problem_id: usize) -> Result<Problem>
{
    for tmp in problems.clone().into_iter()
    {
        if tmp.id == problem_id
        {
            return Ok(tmp)
        }
    }
    return Err(Error::new(ErrorKind::NotFound, "No Such Problem"))
}