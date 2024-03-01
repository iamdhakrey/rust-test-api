use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;
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

    log::info!("Starting Web Server {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("excample %a %{User-Agent}i"))
            .configure(init_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
