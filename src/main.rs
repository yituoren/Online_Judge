use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger;
use log;
use serde::Deserialize;

mod arg;
mod globals;
mod api;

// DO NOT REMOVE: used in automatic testing
#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit() -> impl Responder {
    log::info!("Shutdown as requested");
    std::process::exit(0);
    format!("Exited")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let (config, flush) = arg::get_arg()?;
    println!("{:?}, {}", config, flush);
    let address = config.server.bind_address.clone();
    let port = config.server.bind_port;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(config.clone()))
            .service(api::job::post_jobs)
            // DO NOT REMOVE: used in automatic testing
            .service(exit)
    })
    .bind((address, port))?
    .run()
    .await
}
