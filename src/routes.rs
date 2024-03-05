use crate::{
    api::{echo, hello, manual_hello},
    // json,
};
use actix_web::{http::StatusCode, web, HttpResponse};
use serde_json::json;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(echo);
    cfg.service(web::resource("/hey").to(manual_hello));
    cfg.service(
        web::resource("/json")
            .route(web::get().to(crate::json::hello))
            .route(web::put().to(|| async {
                HttpResponse::build(StatusCode::METHOD_NOT_ALLOWED) //.body("Method Not Allowed")
                    .json(json!({"error": "Method Not Allowed"}))
                // .finish()
            })),
    );
}
