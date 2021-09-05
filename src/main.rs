//! Automated reporting and wiki website for the [Foxhole](https://store.steampowered.com/app/505460/Foxhole/) game

#![feature(non_ascii_idents)]
#![feature(exclusive_range_pattern)]
#![deny(unsafe_code)]

pub mod map;
pub mod models;
pub mod routes;
pub mod schemas;

mod error;

pub use error::{Error, Result};

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::{info, trace};
use simplelog::*;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, fs::File};

/// Address to bind to
const BIND_ADDR: (&str, u16) = ("0.0.0.0", 3224);

#[tokio::main]
async fn main() {
    dotenv().ok();

    // logging
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            match env::var("LOG_TRACE") {
                Ok(_) => LevelFilter::Trace,
                Err(_) => LevelFilter::Info,
            },
            Config::default(),
            File::create("instance.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("Starting holenews instance");

    // load/save resources

    // config
    info!("Loading database");
    trace!("Getting database url from environment variables");
    let db_url = &env::var("DATABASE_URL").expect("Couldn't find DATABASE_URL");
    trace!("Loading pool from new SQLITE connection");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Couldn't connect to pool");

    // run actix
    info!("Starting web server");
    HttpServer::new(move || App::new().data(pool.clone()).configure(routes::init))
        .bind(BIND_ADDR)
        .expect("Couldn't bind server")
        .run()
        .await
        .expect("Server failed")
}
