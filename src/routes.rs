//! Contains actix-based routes used for responding to user requests

use crate::models::War;
use crate::{models::Battle, schemas::Schema, Result};
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use log::{info, trace};
use sqlx::SqlitePool;

/// Factories all routes together for an actix app
pub fn init(cfg: &mut ServiceConfig) {
    trace!("Configuring routes");
    cfg.service(index);
    cfg.service(get_war);
    cfg.service(get_battle);
}

/// Generalized data to populate homepage with
#[get("/api/homepop")]
async fn index(pool: web::Data<SqlitePool>) -> Result<impl Responder> {
    info!("GET for homepage");
    Ok(HttpResponse::Ok().json(
        Schema::default()
            .add_battles(Battle::get_homepage(pool.get_ref()).await?)
            .wars_from_battles(pool.get_ref())
            .await?,
    ))
}

/// Gets war by it's number
#[get("/api/war/{num}")]
async fn get_war(
    pool: web::Data<SqlitePool>,
    web::Path(num): web::Path<usize>,
) -> Result<impl Responder> {
    info!("GET for war of num {}", num);
    Ok(HttpResponse::Ok()
        .json(Schema::default().add_war(War::get_ensure(pool.get_ref(), num as i64).await?)))
}

/// Gets battle by it's id
#[get("/api/battle/{id}")]
async fn get_battle(
    pool: web::Data<SqlitePool>,
    web::Path(id): web::Path<usize>,
) -> Result<impl Responder> {
    info!("GET for battle of id {}", id);
    Ok(HttpResponse::Ok().json(
        Schema::default()
            .add_battle(
                Battle::get_ensure(pool.get_ref(), id as i64)
                    .await?
                    .get_pop_reports(pool.get_ref())
                    .await?,
            )
            .wars_from_battles(pool.get_ref())
            .await?,
    ))
}
