/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use crate::auth;
use crate::hyper;
use actix_web::{
    delete, get, post,
    web::{Bytes, Data, Path},
    HttpRequest, Responder,
};
use std::sync::{Arc, Mutex};

const R_PONG: &str = "PONG";
const R_TRUE: &str = "YES";
const R_FALSE: &str = "NO";
const R_OK: &str = "OK";
const R_INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
const R_AUTH_FAILED: &str = "AUTH_FAILED";

pub fn version() -> String {
    format!(
        "[HyperDB v{} (https://afaan.dev)]",
        option_env!("CARGO_PKG_VERSION").unwrap()
    )
}

#[get("/")]
pub async fn index() -> impl Responder {
    version()
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    R_PONG
}

#[post("/auth")]
pub async fn authenticate(auth: Data<auth::Auth>, req: HttpRequest) -> impl Responder {
    let res = auth::token(auth.into_inner(), req);

    if res == "" {
        return R_INVALID_CREDENTIALS.to_string();
    }

    res
}

#[get("/has/{key}")]
pub async fn has(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
    key: Path<String>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    if hs.lock().unwrap().has(&key) {
        R_TRUE
    } else {
        R_FALSE
    }
}

#[get("/data/{key}")]
pub async fn get(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
    key: Path<String>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED.to_string();
    }

    format!("{}", hs.lock().unwrap().get(&key))
}

#[post("/data/{key}")]
pub async fn set(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
    key: Path<String>,
    bytes: Bytes,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED.to_string();
    }

    hs.lock()
        .unwrap()
        .set(&key, &String::from_utf8(bytes.to_vec()).unwrap());
    format!("{}", hs.lock().unwrap().get(&key))
}

#[delete("/data/{key}")]
pub async fn delete(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
    key: Path<String>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    hs.lock().unwrap().delete(&key);
    R_OK
}

#[get("/data")]
pub async fn all(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED.to_string();
    }

    format!("{}", hs.lock().unwrap().all())
}

#[delete("/data")]
pub async fn clear(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    hs.lock().unwrap().clear();
    R_OK
}

#[get("/empty")]
pub async fn empty(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    if hs.lock().unwrap().is_empty() {
        R_TRUE
    } else {
        R_FALSE
    }
}

#[post("/save")]
pub async fn save(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    hs.lock().unwrap().save_to_file();
    R_OK
}

#[post("/reload")]
pub async fn reload(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    hs.lock().unwrap().reload();
    R_OK
}

#[delete("/reset")]
pub async fn reset(
    auth: Data<auth::Auth>,
    req: HttpRequest,
    hs: Data<Arc<Mutex<hyper::HyperStore>>>,
) -> impl Responder {
    if !auth::check(auth.into_inner(), req) {
        return R_AUTH_FAILED;
    }

    hs.lock().unwrap().reset();
    R_OK
}
