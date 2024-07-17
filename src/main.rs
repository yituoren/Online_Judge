use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use api::job::{job_consumer, job_producer, Job};
use env_logger;
use log;
use tokio::{task, sync::{mpsc, oneshot, Mutex}};
use std::sync::Arc;

mod arg;
mod globals;
mod api;

// DO NOT REMOVE: used in automatic testing
#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit(/*shutdown_signal: web::Data<Arc<Mutex<Option<oneshot::Sender<()>>>>>*/) -> impl Responder {
    log::info!("Shutdown as requested");
    /*if let Some(shutdown_sender) = shutdown_signal.lock().await.take() {
        let _ = shutdown_sender.send(());
    }*/
    std::process::exit(0);
    format!("Exited")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let (config, flush) = arg::get_arg()?;
    let address = config.server.bind_address.clone();
    let port = config.server.bind_port;

    /*let (tx, rx) = mpsc::channel::<Job>(32);
    task::spawn(job_producer(tx, config.clone()));
    task::spawn(job_consumer(rx));*/

    /*let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let shutdown_signal: Arc<Mutex<Option<oneshot::Sender<()>>>> = Arc::new(Mutex::new(Some(shutdown_sender)));*/

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.app_data(web::Data::new(config.clone()))
            //.app_data(web::Data::new(shutdown_signal.clone()))
            //.service(api::job::post_jobs)
            //.service(api::job::get_jobs)
            // DO NOT REMOVE: used in automatic testing
            .service(exit)
    })
    .bind((address, port))?
    .run().await

    /*tokio::select! {
        _ = server => {
            //info!("Server has stopped");
        }
        _ = shutdown_receiver => {
            //info!("Received shutdown signal, shutting down server.");
        }
    };

    Ok(())*/
}
