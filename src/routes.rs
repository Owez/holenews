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
        num: web::Path<usize>,
    ) -> Result<impl Responder> {
        let num = num.into_inner();
        info!("GET for api war of num {} route", num);
        let pool = pool.get_ref();
        Ok(HttpResponse::Ok()
            .json(Schema::default().add_war(War::get_ensure(pool, num as i64).await?)))
    }

    /// Gets battle by it's id
    #[get("/api/battle/{id}")]
    pub async fn get_battle(
        pool: web::Data<SqlitePool>,
        id: web::Path<usize>,
    ) -> Result<impl Responder> {
        let id = id.into_inner();
        info!("GET for api battle of id {} route", id);
        let pool = pool.get_ref();
        Ok(HttpResponse::Ok().json(
            Schema::default()
                .add_battle(
                    Battle::get_ensure(pool, id as i64)
                        .await?
                        .get_pop_reports(pool)
                        .await?,
                )
                .wars_from_battles(pool)
                .await?,
        ))
    }
}

/// Basic routes with data handling, just returning almost always static files
mod basic {
    use crate::{Error, Result};
    use actix_files::NamedFile;
    use actix_web::{get, web, Responder};
    use log::info;
    use std::path::PathBuf;

    /// Homepage showing top information
    #[get("/")]
    pub async fn index() -> Result<impl Responder> {
        info!("GET for basic index route");
        let path: PathBuf = "./static/html/index.html".parse().unwrap();
        NamedFile::open(path).map_err(|err| Error::StaticOpen(err))
    }

    #[get("/battle/{id}")]
    pub async fn battle(id: web::Path<usize>) -> Result<impl Responder> {
        let id = id.into_inner();
        info!("GET for basic battle of id {} route", id);
        let path: PathBuf = "./static/html/battle.html".parse().unwrap();
        NamedFile::open(path).map_err(|err| Error::StaticOpen(err))
    }

    #[get("/war/{num}")]
    pub async fn war(num: web::Path<usize>) -> Result<impl Responder> {
        let num = num.into_inner();
        info!("GET for war battle of num {} route", num);
        let path: PathBuf = "./static/html/war.html".parse().unwrap();
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
    cfg.service(basic::battle);
    cfg.service(basic::war);

    trace!("Configuring static file routes");
    cfg.service(
        Files::new("/static", "./static")
            .show_files_listing()
            .prefer_utf8(true),
    );
}
