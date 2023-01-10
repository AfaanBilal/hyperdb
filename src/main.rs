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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hs = Data::new(Arc::new(Mutex::new(hyper::HyperStore::new("store.hyper"))));

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
