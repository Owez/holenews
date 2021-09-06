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

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use log::{error, info, trace};
use simplelog::*;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, fs::File, process};
use tera::Tera;

/// Address to bind to
const BIND_ADDR: (&str, u16) = ("0.0.0.0", 3224);

#[actix_web::main]
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

    // load database
    info!("Loading database");
    trace!("Getting database url from environment variables");
    let db_url = &env::var("DATABASE_URL").expect("Couldn't find DATABASE_URL");
    trace!("Loading sqlx pool from new sqlite connection");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Couldn't connect to pool");

    // init terra
    trace!("Constructing tera templating instance");
    let tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")) {
        Ok(tera) => tera,
        Err(err) => {
            error!("Could not construct tera templating instance, {}", err);
            process::exit(1)
        }
    };

    // run actix
    println!("Starting web server at {}..", bind_url()); // on purpose
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .app_data(Data::new(pool.clone()))
            .configure(routes::init)
    });
    match server.bind(BIND_ADDR) {
        Ok(server) => match server.run().await {
            Ok(()) => info!("Server ended successfully"),
            Err(_) => {
                error!("Could not run server at {}", bind_url());
                process::exit(1)
            }
        },
        Err(_) => {
            error!("Could not bind server to {}", bind_url());
            process::exit(1)
        }
    }
}

/// Generates url/address string from known bind address
fn bind_url() -> String {
    format!("http://{}:{} address", BIND_ADDR.0, BIND_ADDR.1)
}
