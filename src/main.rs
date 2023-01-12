mod auth;
/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
mod hyper;
mod server;
use std::{
    env,
    sync::{Arc, Mutex},
};

use actix_web::{web::Data, App, HttpServer};

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8765;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hs = Data::new(Arc::new(Mutex::new(hyper::HyperStore::new("store.hyper"))));

    let host: String = env::var("HYPERDB_HOST").unwrap_or(DEFAULT_HOST.to_string());
    let port: u16 = env::var("HYPERDB_PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_PORT);

    let version = server::version();
    println!("{version}: Server starting on {host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(hs.clone())
            .app_data(Data::new(auth::new()).clone())
            .service(server::index)
            .service(server::ping)
            .service(server::authenticate)
            .service(server::has)
            .service(server::get)
            .service(server::set)
            .service(server::delete)
            .service(server::all)
            .service(server::clear)
            .service(server::empty)
            .service(server::save)
            .service(server::reload)
            .service(server::reset)
    })
    .bind((host, port))?
    .run()
    .await
}
