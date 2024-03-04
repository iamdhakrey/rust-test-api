use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::{env, sync::Mutex};

mod api;
mod json;
mod logger;
mod routes;
use routes::init_routes;

use logger::logs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize logger
    logs();
    dotenv().ok();

    // get host from environment variable
    let host: String = env::var("HOST").expect("HOST must be set");

    // get port from environment variable and must in integer
    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a number");

    // get workers from environment variable and must in integer
    // and must be greater than 0 and by default is 2
    let workers: usize = env::var("WORKERS")
        .unwrap_or("2".to_string())
        .parse()
        .expect("WORKERS must be a number and greater than 0");

    log::info!(
        "Starting Web Server http://{}:{} with {} workers",
        host,
        port,
        workers
    );

    let counter = web::Data::new(api::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(init_routes)
            .app_data(counter.clone())
            .route("/count", web::get().to(api::index))
    })
    // actix web server will run with 1 worker
    .workers(workers)
    .bind((host, port))?
    .run()
    .await
}
