use std::cmp::Ordering;
use std::collections::HashSet;
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::arg::Config;
use crate::globals::{CONTEST_LIST, JOB_LIST, USER_LIST};
use crate::api::error::HttpError;
use crate::api::user::User;

use super::job::Job;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRank
{
    pub user: User,
    pub rank: usize,
    pub scores: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FullUserInfo
{
    pub user_id: usize,
    pub problems: Vec<Vec<Job>>,
    pub scores: Vec<f64>,
    pub times: Vec<String>,
    pub count: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RankQuery
{
    pub scoring_rule: Option<String>,
    pub tie_breaker: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contest
{
    pub id: usize,
    pub name: String,
    pub from: String,
    pub to: String,
    pub problem_ids: Vec<usize>,
    pub user_ids: Vec<usize>,
    pub submission_limit: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostContest
{
    pub id: Option<usize>,
    pub name: String,
    pub from: String,
    pub to: String,
    pub problem_ids: Vec<usize>,
    pub user_ids: Vec<usize>,
    pub submission_limit: usize,
}

pub fn rank(scoring_rule: &str, x: &FullUserInfo) -> FullUserInfo
{
    let mut a = x.clone();
    match scoring_rule
    {
        "latest" =>
        {
            for jobs in a.problems.iter()
            {
                if jobs.is_empty()
                {
                    a.scores.push(0.0);
                    a.times.push("".to_string())
                }
                else
                {
                    let job = jobs.last().unwrap();
                    a.scores.push(job.score);
                    a.times.push(job.created_time.clone());
                    a.count += jobs.len();
                }
            }
        }
        _ =>
        {
            for jobs in a.problems.iter_mut()
            {
                if jobs.is_empty()
                {
                    a.scores.push(0.0);
                    a.times.push("".to_string());
                }
                else
                {
                    jobs.sort_by(|x, y| x.score.total_cmp(&y.score));
                    let job = jobs.last().unwrap();
                    a.scores.push(job.score);
                    a.times.push(job.created_time.clone());
                    a.count += jobs.len();
                }
            }
        }
    }
    a
}

pub fn cmp_rank(tie_breaker: &str, a: &FullUserInfo, b: &FullUserInfo) -> Ordering
{
    let a_total: f64 = a.scores.iter().sum();
    let b_total: f64 = b.scores.iter().sum();
    if a_total > b_total
    {
        return Ordering::Less;
    }
    else if a_total < b_total
    {
        return Ordering::Greater;
    }
    match tie_breaker
    {
        "submission_time" =>
        {
            let mut a_time = a.times.iter().max().unwrap().clone();
            if a_time == ""
            {
                a_time = "9".to_string();
            }
            let mut b_time = b.times.iter().max().unwrap().clone();
            if b_time == ""
            {
                b_time = "9".to_string();
            }
            return a_time.cmp(&b_time);
        }
        "submission_count" =>
        {
            return a.count.cmp(&b.count);
        }
        "user_id" =>
        {
            return a.user_id.cmp(&b.user_id);
        }
        _ =>
        {
            return Ordering::Equal;
        }
    }
}

#[get("/contests/{contestid}/ranklist")]
pub async fn get_contests_ranklist(get_contest: web::Path<usize>, query: web::Query<RankQuery>, config: web::Data<Config>) -> HttpResponse
{
    let lock = USER_LIST.lock().await;
    let user_list = lock.clone();
    drop(lock);
    let lock = JOB_LIST.lock().await;
    let job_list = lock.clone();
    drop(lock);

    let id = *get_contest;
    let user_count = user_list.len();
    let mut full_rank: Vec<FullUserInfo> = Vec::new();
    if id == 0
    {
        for user_id in 0..user_count
        {
            let mut user = FullUserInfo {
                user_id,
                problems: Vec::new(),
                scores: Vec::new(),
                times: Vec::new(),
                count: 0,
            };
            for problem in config.problems.iter()
            {
                let jobs: Vec<Job> = job_list.iter()
                    .filter(|x| x.submission.problem_id == problem.id && x.submission.user_id == user_id)
                    .cloned()
                    .collect();
                user.problems.push(jobs);
            }
            full_rank.push(user);
        }
    }
    else
    {
        match CONTEST_LIST.lock().await.get(id - 1)
        {
            Some(contest) =>
            {
                for user_id in contest.user_ids.iter()
                {
                    let mut user = FullUserInfo {
                        user_id: *user_id,
                        problems: Vec::new(),
                        scores: Vec::new(),
                        times: Vec::new(),
                        count: 0,
                    };
                    for problem_id in contest.problem_ids.iter()
                    {
                        let jobs: Vec<Job> = job_list.iter()
                            .filter(|x| x.submission.problem_id == *problem_id && x.submission.user_id == *user_id)
                            .cloned()
                            .collect();
                        user.problems.push(jobs);
                    }
                    full_rank.push(user);
                }
            }
            None =>
            {
                return HttpResponse::NotFound()
                    .content_type("application/json")
                    .json(HttpError {
                        code: 3,
                        reason: "ERR_NOT_FOUND".to_string(),
                        message: "Contest ".to_string() + &id.to_string() + " not found.",
                    });
            }
        }
    }

    let mut scoring_rule = "last".to_string();
    if let Some(rule) = &query.scoring_rule
    {
        scoring_rule = rule.to_string();
    }
    let mut tie_breaker = String::new();
    if let Some(rule) = &query.tie_breaker
    {
        tie_breaker = rule.to_string();
    }

    let mut after_rank: Vec<FullUserInfo> = full_rank.iter()
        .map(|x| rank(&scoring_rule, x))
        .collect();
    after_rank.sort_by(|x, y| cmp_rank(&tie_breaker, x, y));

    let mut ranklist: Vec<UserRank> = Vec::new();
    let mut count: usize = 1;
    let mut rank: usize = 1;
    let mut last: FullUserInfo = FullUserInfo {
        user_id: 0,
        problems: Vec::new(),
        scores: Vec::new(),
        times: Vec::new(),
        count: 0,
    };
    for user in after_rank.iter()
    {
        if cmp_rank(&tie_breaker, &last, user) != Ordering::Equal
        {
            rank = count;
        }
        ranklist.push(UserRank {
            user: user_list[user.user_id].clone(),
            rank,
            scores: user.scores.clone(),
        });
        last = user.clone();
        count += 1;
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ranklist)
}

#[post("/contests")]
pub async fn post_contests(post_contest: web::Json<PostContest>, config: web::Data<Config>) -> HttpResponse
{
    if post_contest.from >= post_contest.to
    {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(HttpError {
                code: 1,
                reason: "ERR_INVALID_ARGUMENT".to_string(),
                message: "Invalid argument time.".to_string(),
            });
    }
    let user_list = USER_LIST.lock().await;
    let user_amount = user_list.len();
    drop(user_list);
    let mut user_set: HashSet<usize> = HashSet::new();
    for user_id in post_contest.user_ids.iter()
    {
        if !user_set.insert(*user_id)
        {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .json(HttpError {
                    code: 1,
                    reason: "ERR_INVALID_ARGUMENT".to_string(),
                    message: "Invalid argument user.".to_string(),
                });
        }
        else if *user_id >= user_amount
        {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .json(HttpError {
                    code: 3,
                    reason: "ERR_NOT_FOUND".to_string(),
                    message: "User ".to_string() + &user_id.to_string() + " not found.",
                });
        }
    }
    let problem_amount = config.problems.len();
    let mut problem_set: HashSet<usize> = HashSet::new();
    for problem_id in post_contest.problem_ids.iter()
    {
        if !problem_set.insert(*problem_id)
        {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .json(HttpError {
                    code: 1,
                    reason: "ERR_INVALID_ARGUMENT".to_string(),
                    message: "Invalid argument problem.".to_string(),
                });
        }
        else if *problem_id >= problem_amount
        {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .json(HttpError {
                    code: 3,
                    reason: "ERR_NOT_FOUND".to_string(),
                    message: "Problem ".to_string() + &problem_id.to_string() + " not found.",
                });
        }
    }
    let mut lock = CONTEST_LIST.lock().await;
    let max = lock.len();
    if let Some(id) = post_contest.id
    {
        if id > max
        {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .json(HttpError {
                    code: 3,
                    reason: "ERR_NOT_FOUND".to_string(),
                    message: "Contest ".to_string() + &id.to_string() + " not found.",
                });
        }
        else if id == 0
        {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .json(HttpError {
                    code: 1,
                    reason: "ERR_INVALID_ARGUMENT".to_string(),
                    message: "Invalid contest id.".to_string(),
                });
        }
        else
        {
            lock[id - 1] = Contest {
                id,
                name: post_contest.name.clone(),
                from: post_contest.from.clone(),
                to: post_contest.to.clone(),
                problem_ids: post_contest.problem_ids.clone(),
                user_ids: post_contest.user_ids.clone(),
                submission_limit: post_contest.submission_limit.clone(),
            };
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(lock[id - 1].clone());
        }
    }
    lock.push(Contest {
        id: max + 1,
        name: post_contest.name.clone(),
        from: post_contest.from.clone(),
        to: post_contest.to.clone(),
        problem_ids: post_contest.problem_ids.clone(),
        user_ids: post_contest.user_ids.clone(),
        submission_limit: post_contest.submission_limit.clone(),
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .json(lock[max].clone())
}

#[get("/contests/{contestid}")]
pub async fn get_contests_id(get_contest: web::Path<usize>) -> HttpResponse
{
    if *get_contest == 0
    {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(HttpError {
                code: 1,
                reason: "ERR_INVALID_ARGUMENT".to_string(),
                message: "Invalid contest id.".to_string(),
            });
    }
    match CONTEST_LIST.lock().await.get(*get_contest - 1)
    {
        Some(contest) =>
        {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(contest);
        }
        None =>
        {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .json(HttpError {
                    code: 3,
                    reason: "ERR_NOT_FOUND".to_string(),
                    message: "Contest ".to_string() + &get_contest.to_string() + " not found.",
                })
        }
    }
}

#[get("/contests")]
pub async fn get_contests() -> HttpResponse
{
    HttpResponse::Ok()
        .content_type("application/json")
        .json(CONTEST_LIST.lock().await.clone())
}