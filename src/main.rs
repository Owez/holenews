//! Automated reporting and wiki website for the [Foxhole](https://store.steampowered.com/app/505460/Foxhole/) game

#![feature(non_ascii_idents)]
#![deny(unsafe_code)]

pub mod routes;
pub mod map;

mod battle;
mod population;
mod war;
mod error;

pub use battle::Battle;
pub use population::Population;
pub use war::War;
pub use error::{Error,Result};

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

/// Address to bind to
const BIND_ADDR: (&str, u16) = ("0.0.0.0", 3224);

#[tokio::main]
async fn main() {
    // load/save resources
    dotenv().ok();

    // config
    let db_url = &env::var("DATABASE_URL").expect("Couldn't find DATABASE_URL");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Couldn't connect to pool");

    // run actix
    HttpServer::new(move || App::new().data(pool.clone()).configure(routes::init))
        .bind(BIND_ADDR)
        .expect("Couldn't bind server")
        .run()
        .await
        .expect("Server failed")
}
