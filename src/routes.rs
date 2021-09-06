//! Contains actix-based routes used for responding to user requests

use crate::models::War;
use crate::{models::Battle, schemas::Schema, Result};
use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use log::{info, trace};
use sqlx::SqlitePool;

#[get("/")]
async fn index(pool: web::Data<SqlitePool>) -> Result<impl Responder> {
    info!("GET of index/homepage route");
    let pool = pool.get_ref();
    let _schema = Schema::default()
        .add_battles(Battle::get_homepage(pool).await?)
        .wars_from_battles(pool)
        .await?;
    Ok(HttpResponse::Ok().body("tera here soon"))
}

#[get("/battle/{id}")]
async fn battle(pool: web::Data<SqlitePool>, id: web::Path<usize>) -> Result<impl Responder> {
    let id = id.into_inner();
    info!("GET of battle of id {} route", id);
    let pool = pool.get_ref();
    let _schema = Schema::default()
        .add_battle(Battle::get_ensure(pool, id as i64).await?)
        .wars_from_battles(pool)
        .await?;
    Ok(HttpResponse::Ok().body("tera here soon"))
}

#[get("/war/{num}")]
async fn war(pool: web::Data<SqlitePool>, num: web::Path<usize>) -> Result<impl Responder> {
    let num = num.into_inner();
    info!("GET of war of num {} route", num);
    let pool = pool.get_ref();
    let _schema = Schema::default().add_war(War::get_ensure(pool, num as i64).await?);
    Ok(HttpResponse::Ok().body("tera here soon"))
}

/// Factories all routes together for an actix app
pub fn init(cfg: &mut ServiceConfig) {
    trace!("Configuring typical routes");
    cfg.service(index);
    cfg.service(battle);
    cfg.service(war);

    trace!("Configuring static file routes");
    cfg.service(
        Files::new("/static", "./static")
            .show_files_listing()
            .prefer_utf8(true),
    );
}
