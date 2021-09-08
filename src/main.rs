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
use std::collections::{BTreeMap, HashMap};
use std::{env, fs::File, process};
use tera::{self, Tera};

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
    let mut tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")) {
        Ok(tera) => tera,
        Err(err) => {
            error!("Could not construct tera templating instance, {}", err);
            process::exit(1)
        }
    };
    tera.register_function("url_for", make_url_for(route_urls()));

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

/// Taken from [Tera Docs](https://tera.netlify.app/docs/#introduction), allows `url_for` mapping of templates
fn make_url_for(urls: BTreeMap<String, String>) -> impl tera::Function {
    Box::new(
        move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
            match args.get("name") {
                Some(val) => match tera::from_value::<String>(val.clone()) {
                    Ok(v) => {
                        let mut output = urls.get(&v).unwrap().to_string();
                        if let Some(extra) = args.get("extra") {
                            output.push_str(&format!("/{}", extra))
                        }

                        Ok(tera::to_value(output).unwrap())
                    }
                    Err(_) => Err("oops".into()), // TODO: update
                },
                None => Err("oops".into()), // TODO: update
            }
        },
    )
}

/// Returns urls used for routes, inglorious due to tera
fn route_urls() -> BTreeMap<String, String> {
    let mut urls = BTreeMap::new();
    urls.insert("index".to_string(), "/".to_string());
    urls.insert("battle".to_string(), "/battle".to_string());
    urls.insert("war".to_string(), "/war".to_string());
    urls
}
