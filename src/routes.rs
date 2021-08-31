//! Contains actix-based routes used for responding to user requests

use crate::{models::Battle, schemas::Schema, Result};
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use sqlx::SqlitePool;

/// Factories all routes together for an actix app
pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

/// Homepage population
#[get("/api/homepop")]
async fn index(pool: web::Data<SqlitePool>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(
        Schema::default()
            .add_battles(Battle::get_homepage(pool.get_ref()).await?)
            .wars_from_battles(pool.get_ref())
            .await?,
    ))
}
