// use crate::src::{echo, hello};

// use test_api::{echo, hello};
#[cfg(test)]
mod tests {
    // use super::*;
    use actix_web::{test, App};
    use test_api::{echo, hello};
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
