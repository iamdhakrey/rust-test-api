use actix_web::{HttpResponse, Responder};
use serde_json::json;
pub async fn hello() -> impl Responder {
    let body = json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null
        }
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(body)
}
