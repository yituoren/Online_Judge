use rusqlite::{Result, params};

use crate::globals::{CONTEST_LIST, DATABASE, JOB_LIST, USER_LIST};
use crate::api::contest::Contest;
use crate::api::job::Job;
use crate::api::user::User;

pub async fn drop_all_tables() -> Result<()>
{
    let database = DATABASE.lock().await;

    let table_names: Vec<String> = database.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;

    for table_name in table_names
    {
        let drop_sql = format!("DROP TABLE IF EXISTS {}", table_name);
        database.execute(&drop_sql, [])?;
    }

    Ok(())
}

pub async fn create_tables() -> Result<()>
{
    let database = DATABASE.lock().await;
    database.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY,
            created_time TEXT NOT NULL,
            updated_time TEXT NOT NULL,
            submission TEXT NOT NULL,
            state TEXT NOT NULL,
            result TEXT NOT NULL,
            score REAL NOT NULL,
            cases TEXT NOT NULL
         )",
        [],
    )?;
    
    database.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
         )",
        [],
    )?;
    
    database.execute(
        "CREATE TABLE IF NOT EXISTS contests (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            from_time TEXT NOT NULL,
            to_time TEXT NOT NULL,
            problem_ids TEXT NOT NULL,
            user_ids TEXT NOT NULL,
            submission_limit INTEGER NOT NULL
         )",
        [],
    )?;

    Ok(())
}

pub async fn read_jobs() -> Result<()>
{
    let database = DATABASE.lock().await;
    let jobs: Result<Vec<Job>> = database.prepare(
        "SELECT id, created_time, updated_time, submission, state, result, score, cases FROM jobs")?
        .query_map([], |row| {
            Ok(Job {
                id: row.get(0)?,
                created_time: row.get(1)?,
                updated_time: row.get(2)?,
                submission: serde_json::from_str(row.get::<_, String>(3)?.as_str()).expect("Failed to read cases"),
                state: row.get(4)?,
                result: row.get(5)?,
                score: row.get(6)?,
                cases: serde_json::from_str(row.get::<_, String>(7)?.as_str()).expect("Failed to read cases."),
                })
        })?
        .collect();

    let mut lock = JOB_LIST.lock().await;
    *lock = jobs?;
    println!("{:?}", lock);

    Ok(())
}

pub async fn read_contests() -> Result<()>
{
    let database = DATABASE.lock().await;
    let contests: Result<Vec<Contest>> = database.prepare(
        "SELECT id, name, from_time, to_time, problem_ids, user_ids, submission_limit FROM contests")?
        .query_map([], |row| {
            Ok(Contest {
                id: row.get(0)?,
                name: row.get(1)?,
                from: row.get(2)?,
                to: row.get(3)?,
                problem_ids: serde_json::from_str(row.get::<_, String>(4)?.as_str()).expect("Failed to read problem_ids"),
                user_ids: serde_json::from_str(row.get::<_, String>(5)?.as_str()).expect("Failed to read user_ids."),
                submission_limit: row.get(6)?,
                })
        })?
        .collect();

    let mut lock = CONTEST_LIST.lock().await;
    *lock = contests?;
    println!("{:?}", lock);
    
    Ok(())
}

pub async fn read_users() -> Result<()>
{
    let database = DATABASE.lock().await;
    let users: Result<Vec<User>> = database.prepare(
        "SELECT id, name FROM users")?
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                })
        })?
        .collect();

    let mut lock = USER_LIST.lock().await;
    *lock = users?;
    if lock.is_empty()
    {
        let user = User {
            id: 0,
            name: "root".to_string(),
        };
        lock.push(user.clone());
        let _ = database.execute(
            "INSERT INTO users (id, name) VALUES (?1, ?2)",
            params![
                user.id,
                user.name,
            ]
        );
    }
    println!("{:?}", lock);
    
    Ok(())
}

pub async fn delete_job(job_id: usize) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "DELETE FROM jobs WHERE id = ?1",
        params![job_id],
    )
}

pub async fn update_job(job: &Job) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "UPDATE jobs SET updated_time = ?1, state = ?2, result = ?3, score = ?4, cases = ?5 WHERE id = ?6",
        params![
            job.updated_time,
            job.state,
            job.result,
            job.score,
            serde_json::to_string(&job.cases).unwrap(),
            job.id,
        ]
    )
}

pub async fn insert_job(job: &Job) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "INSERT INTO jobs (id, created_time, updated_time, submission, state, result, score, cases) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            job.id,
            job.created_time,
            job.updated_time,
            serde_json::to_string(&job.submission).unwrap(),
            job.state,
            job.result,
            job.score,
            serde_json::to_string(&job.cases).unwrap(),
        ]
    )
}

pub async fn update_user(user: &User) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "UPDATE users SET name = ?1 WHERE id = ?2",
        params![
            user.name,
            user.id,
        ]
    )
}

pub async fn insert_user(user: &User) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "INSERT INTO users (id, name) VALUES (?1, ?2)",
        params![
            user.id,
            user.name,
        ]
    )
}

pub async fn update_contest(contest: &Contest) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "UPDATE contests SET name = ?1, from_time = ?2, to_time = ?3, problem_ids = ?4, user_ids = ?5, submission_limit = ?6 WHERE id = ?7",
        params![
            contest.name,
            contest.from,
            contest.to,
            serde_json::to_string(&contest.problem_ids).unwrap(),
            serde_json::to_string(&contest.user_ids).unwrap(),
            contest.submission_limit,
            contest.id,
        ]
    )
}

pub async fn insert_contest(contest: &Contest) -> Result<usize>
{
    let database = DATABASE.lock().await;
    database.execute(
        "INSERT INTO contests (id, name, from_time, to_time, problem_ids, user_ids, submission_limit) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            contest.id,
            contest.name,
            contest.from,
            contest.to,
            serde_json::to_string(&contest.problem_ids).unwrap(),
            serde_json::to_string(&contest.user_ids).unwrap(),
            contest.submission_limit,
        ]
    )
}