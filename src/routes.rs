//! Contains actix-based routes used for responding to user requests

use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

/// Factories all routes together for an actix app
pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

/// Homepage
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("TODO")
}
