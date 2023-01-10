/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use crate::hyper;

use actix_web::{
    delete, get, post,
    web::{Bytes, Data, Path},
    Responder,
};
use std::sync::{Arc, Mutex};

#[get("/")]
pub async fn index() -> impl Responder {
    let version = option_env!("CARGO_PKG_VERSION").unwrap();
    format!("[HyperDB v{version} (https://afaan.dev)]")
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    format!("PONG")
}

#[get("/has/{key}")]
pub async fn has(hs: Data<Arc<Mutex<hyper::HyperStore>>>, key: Path<String>) -> impl Responder {
    let has = hs.lock().unwrap().has(&key);
    if has {
        format!("YES")
    } else {
        format!("NO")
    }
}

#[get("/data/{key}")]
pub async fn get(hs: Data<Arc<Mutex<hyper::HyperStore>>>, key: Path<String>) -> impl Responder {
    let value = hs.lock().unwrap().get(&key);
    format!("{value}")
}

#[post("/data/{key}")]
pub async fn set(
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
    key: Path<String>,
    bytes: Bytes,
) -> impl Responder {
    hs.lock()
        .unwrap()
        .set(&key, &String::from_utf8(bytes.to_vec()).unwrap());
    let value = hs.lock().unwrap().get(&key);
    format!("{value}")
}

#[delete("/data/{key}")]
pub async fn delete(hs: Data<Arc<Mutex<hyper::HyperStore>>>, key: Path<String>) -> impl Responder {
    hs.lock().unwrap().delete(&key);
    format!("OK")
}

#[get("/data")]
pub async fn all(hs: Data<Arc<Mutex<hyper::HyperStore>>>) -> impl Responder {
    let data = hs.lock().unwrap().all();
    format!("{data}")
}

#[delete("/data")]
pub async fn clear(hs: Data<Arc<Mutex<hyper::HyperStore>>>) -> impl Responder {
    hs.lock().unwrap().clear();
    format!("OK")
}

#[get("/empty")]
pub async fn empty(hs: Data<Arc<Mutex<hyper::HyperStore>>>) -> impl Responder {
    let empty = hs.lock().unwrap().is_empty();

    if empty {
        format!("YES")
    } else {
        format!("NO")
    }
}
