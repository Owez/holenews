//! Contains actix-based routes used for responding to user requests

use actix_files::Files;
use actix_web::web::ServiceConfig;
use log::trace;

/// Links for basic routes to hook onto in order to retrieve data
mod api {
    use crate::models::War;
    use crate::{models::Battle, schemas::Schema, Result};
    use actix_web::web;
    use actix_web::{get, HttpResponse, Responder};
    use log::info;
    use sqlx::SqlitePool;

    /// Generalized data to populate homepage with
    #[get("/api/homepop")]
    pub async fn get_homepop(pool: web::Data<SqlitePool>) -> Result<impl Responder> {
        info!("GET for api homepage population route");
        Ok(HttpResponse::Ok().json(
            Schema::default()
                .add_battles(Battle::get_homepage(pool.get_ref()).await?)
                .wars_from_battles(pool.get_ref())
                .await?,
        ))
    }

    /// Gets war by it's number
    #[get("/api/war/{num}")]
    pub async fn get_war(
        pool: web::Data<SqlitePool>,
        web::Path(num): web::Path<usize>,
    ) -> Result<impl Responder> {
        info!("GET for api war of num {} route", num);
        Ok(HttpResponse::Ok()
            .json(Schema::default().add_war(War::get_ensure(pool.get_ref(), num as i64).await?)))
    }

    /// Gets battle by it's id
    #[get("/api/battle/{id}")]
    pub async fn get_battle(
        pool: web::Data<SqlitePool>,
        web::Path(id): web::Path<usize>,
    ) -> Result<impl Responder> {
        info!("GET for api battle of id {} route", id);
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
}

/// Basic routes with data handling, just returning almost always static files
mod basic {
    use crate::{Error, Result};
    use actix_files::NamedFile;
    use actix_web::{get, Responder};
    use log::info;
    use std::path::PathBuf;

    /// Homepage showing top information
    #[get("/")]
    pub async fn index() -> Result<impl Responder> {
        info!("GET for basic index route");
        let path: PathBuf = "./static/html/index.html".parse().unwrap();
        NamedFile::open(path).map_err(|err| Error::StaticOpen(err))
    }
}

/// Factories all routes together for an actix app
pub fn init(cfg: &mut ServiceConfig) {
    trace!("Configuring typical routes");
    cfg.service(api::get_homepop);
    cfg.service(api::get_war);
    cfg.service(api::get_battle);
    cfg.service(basic::index);

    trace!("Configuring static file routes");
    cfg.service(Files::new("/static", ".").prefer_utf8(true));
}
