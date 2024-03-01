use actix_web::web;

use crate::api::{echo, hello, manual_hello};
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(echo);
    cfg.service(web::resource("/hey").to(manual_hello));
}
