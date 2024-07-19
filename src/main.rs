use actix_web::{middleware::Logger, post, web, App, HttpServer, Responder};
use api::{job::{job_consumer, job_producer, Job}, user::User};
use env_logger;
use globals::{CONTEST_LIST, DATABASE, JOB_LIST, USER_LIST};
use log;
use sql::{create_tables, drop_all_tables, read_contests, read_jobs, read_users};
use tokio::{task, sync::mpsc};

mod arg;
mod globals;
mod api;
mod sql;

// DO NOT REMOVE: used in automatic testing
#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit() -> impl Responder
{
    log::info!("Shutdown as requested");
    std::process::exit(0);
    format!("Exited")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (config, flush) = arg::get_arg()?;
    let address = config.server.bind_address.clone();
    let port = config.server.bind_port;
    println!("{}", flush);

    if flush
    {
        let _ = drop_all_tables().await;
    }

    let _ = create_tables().await;
    let _ = read_jobs().await;
    let _ = read_contests().await;
    let _ = read_users().await;

    let (tx, rx) = mpsc::channel::<Job>(32);
    task::spawn(job_producer(tx, config.clone()));
    task::spawn(job_consumer(rx));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(flush))
            .service(api::job::post_jobs)
            .service(api::job::get_jobs_id)
            .service(api::job::get_jobs_query)
            .service(api::job::put_jobs_id)
            .service(api::job::delete_jobs)
            .service(api::user::post_users)
            .service(api::user::get_users)
            .service(api::contest::post_contests)
            .service(api::contest::get_contests)
            .service(api::contest::get_contests_id)
            .service(api::contest::get_contests_ranklist)
            // DO NOT REMOVE: used in automatic testing
            .service(exit)
    })
    .bind((address, port))?
    .run()
    .await

}
