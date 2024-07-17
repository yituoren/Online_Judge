use std::cmp::Ordering;
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::arg::Config;
use crate::globals::{USER_LIST, JOB_LIST};
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
    for user_id in 0..user_count
    {
        let mut user = FullUserInfo {
            user_id,
            problems: Vec::new(),
            scores: Vec::new(),
            times: Vec::new(),
            count: 0,
        };
        if id == 0
        {
            for problem in config.problems.iter()
            {
                let jobs: Vec<Job> = job_list.iter()
                    .filter(|x| x.submission.problem_id == problem.id && x.submission.user_id == user_id)
                    .cloned()
                    .collect();
                user.problems.push(jobs);
            }
        }
        else
        {
            //
        }
        full_rank.push(user);
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