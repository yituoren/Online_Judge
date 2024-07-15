use actix_web::HttpResponse;
use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use std::process::{Command, ExitStatus, Stdio};
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{Error, ErrorKind, Result, Write};

use crate::globals;
use crate::arg::Config;

#[derive(Debug, Deserialize)]
pub struct PostJob
{
    pub source_code: String,
    pub language: String,
    pub user_id: u32,
    pub contest_id: u32,
    pub problem_id: u32,
}

#[derive(Debug, Serialize)]
pub struct Job;

/*pub fn init_jobs(web_service_config: &mut web::ServiceConfig)
{
    web_service_config.service(
        web::resource("/jobs")
        .route(web::post().to(post_jobs)));
}*/

#[post("/jobs")]
pub async fn post_jobs(post_job: web::Json<PostJob>, config: web::Data<Config>) -> HttpResponse
{
    log::info!("Post job: {:?}", post_job);

    let _ = create_dir_all("./tmp_code_runner");
    let path = "./tmp_code_runner/".to_string() + &post_job.user_id.to_string() + "_" + &post_job.problem_id.to_string();

    let mut source_file = File::create(path.clone() + ".rs").unwrap();
    let _ = source_file.write(post_job.source_code.as_bytes());

    match compile_program(path.clone(), post_job.language.clone()).await
    {
        Ok(_) => (),
        Err(_) => (),
    }

    let out_file = File::create(path.clone() + ".out").unwrap();
    match run_program(path.clone(), out_file).await
    {
        Ok(_) => (),
        Err(_) => (),
    }
    
    let _ = remove_dir_all("./tmp_code_runner");
    HttpResponse::Ok().body("OK")
}

async fn run_program(path: String, out_file: File) -> Result<ExitStatus>
{
    Command::new(&path)
                    //.stdin(Stdio::from(in_file))
                    .stdout(Stdio::from(out_file))
                    .stderr(Stdio::null())
                    .status()
}

async fn compile_program(path: String, language: String) -> Result<ExitStatus>
{
    match language.as_str()
    {
        "Rust" =>
        {
            Command::new("rustc")
                .arg(&(path.clone() + ".rs"))
                .arg("-o")
                .arg(path.clone())
                .status()
        }
        _ => Err(Error::new(ErrorKind::Unsupported, "No Such Language"))
    }
}