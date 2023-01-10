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
use std::sync::{Arc, Mutex};

use actix_web::{web::Data, App, HttpServer};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8765;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hs = Data::new(Arc::new(Mutex::new(hyper::HyperStore::new("store.hyper"))));

    let host: &str = match option_env!("HYPERDB_HOST") {
        Some(p) => p,
        None => DEFAULT_HOST,
    };

    let port: u16 = match option_env!("HYPERDB_PORT") {
        Some(p) => p.parse::<u16>().unwrap(),
        None => DEFAULT_PORT,
    };

    let version = server::version();
    println!("{version}: Server starting on {host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(hs.clone())
            .service(server::index)
            .service(server::ping)
            .service(server::has)
            .service(server::get)
            .service(server::set)
            .service(server::delete)
            .service(server::all)
            .service(server::clear)
            .service(server::empty)
    })
    .bind((host, port))?
    .run()
    .await
}
