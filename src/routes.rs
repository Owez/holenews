//! Contains actix-based routes used for responding to user requests

use actix_files::Files;
use actix_web::web::ServiceConfig;
use log::trace;

mod api {
    use crate::{models::Battle, Result};
    use actix_web::{patch, web, HttpResponse, Responder};
    use log::info;
    use serde::Deserialize;
    use sqlx::SqlitePool;

    #[derive(Deserialize)]
    pub struct UpdateBattle {
        name: Option<String>,
        description: Option<String>,
    }

    /// Updates battle of id
    #[patch("/api/battle/{id}")]
    pub async fn patch_battle(
        pool: web::Data<SqlitePool>,
        update: web::Json<UpdateBattle>,
        id: web::Path<usize>,
    ) -> Result<impl Responder> {
        let id = id.into_inner();
        let (name, description) = (update.name.clone(), update.description.clone());
        info!("PATCH of api battle of id {} route", id);

        let pool = pool.get_ref();

        Battle::update(pool, id as i64, name, description).await?;
        Ok(HttpResponse::Ok())
    }
}

mod basic {
    use crate::models::War;
    use crate::{models::Battle, schemas::Schema, Result};
    use actix_web::web;
    use actix_web::{get, HttpResponse, Responder};
    use log::info;
    use sqlx::SqlitePool;

    /// Gets homepage
    #[get("/")]
    pub async fn index(
        pool: web::Data<SqlitePool>,
        tmpl: web::Data<tera::Tera>,
    ) -> Result<impl Responder> {
        info!("GET of index/homepage route");

        let pool = pool.get_ref();

        let tmpl_ctx = Schema::default()
            .add_battles(Battle::get_homepage(pool).await?)
            .wars_from_battles(pool)
            .await?
            .to_tmpl_ctx();
        Ok(HttpResponse::Ok().body(tmpl.render("index.html", &tmpl_ctx)?))
    }

    /// Gets battle of id
    #[get("/battle/{id}")]
    pub async fn battle(
        pool: web::Data<SqlitePool>,
        tmpl: web::Data<tera::Tera>,
        id: web::Path<usize>,
    ) -> Result<impl Responder> {
        let id = id.into_inner();
        info!("GET of battle of id {} route", id);

        let pool = pool.get_ref();

        let tmpl_ctx = Schema::default()
            .add_battle(Battle::get_ensure(pool, id as i64).await?)
            .wars_from_battles(pool)
            .await?
            .to_tmpl_ctx();
        Ok(HttpResponse::Ok().body(tmpl.render("battle.html", &tmpl_ctx)?))
    }

    /// Gets war of number
    #[get("/war/{num}")]
    pub async fn war(
        pool: web::Data<SqlitePool>,
        tmpl: web::Data<tera::Tera>,
        num: web::Path<usize>,
    ) -> Result<impl Responder> {
        let num = num.into_inner();
        info!("GET of war of num {} route", num);

        let pool = pool.get_ref();

        let tmpl_ctx = Schema::default()
            .add_war(War::get_ensure(pool, num as i64).await?)
            .to_tmpl_ctx();
        Ok(HttpResponse::Ok().body(tmpl.render("war.html", &tmpl_ctx)?))
    }
}

/// Factories all routes together for an actix app
pub fn init(cfg: &mut ServiceConfig) {
    trace!("Configuring typical routes");
    cfg.service(api::patch_battle);
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
