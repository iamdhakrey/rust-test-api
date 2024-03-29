use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- access counter inside Mutex
    *counter += 1; // <- increase counter
    format!("Request number: {}", counter) // <- response with new counter value
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;
    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
    }

    #[actix_web::test]
    async fn test_echo() {
        let app = test::init_service(App::new().service(echo)).await;

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload("Test")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Test");
    }
}
