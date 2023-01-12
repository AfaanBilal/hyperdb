/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use actix_web::{http::header::HeaderValue, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, time::SystemTime};

const DEFAULT_SECRET: &str = "";
const DEFAULT_USERNAME: &str = "";
const DEFAULT_PASSWORD: &str = "";

#[derive(Clone)]
pub struct Auth {
    pub enabled: bool,
    secret: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

pub fn new() -> Auth {
    Auth {
        enabled: env::var("HYPERDB_AUTH").is_ok(),
        secret: env::var("HYPERDB_SECRET").unwrap_or(DEFAULT_SECRET.to_string()),
        username: env::var("HYPERDB_USERNAME").unwrap_or(DEFAULT_USERNAME.to_string()),
        password: env::var("HYPERDB_PASSWORD").unwrap_or(DEFAULT_PASSWORD.to_string()),
    }
}

pub fn token(auth: Arc<Auth>, req: HttpRequest) -> String {
    let default_value = HeaderValue::from_str("").unwrap();
    let username = req.headers().get("username").unwrap_or(&default_value);
    let password = req.headers().get("password").unwrap_or(&default_value);

    if username == "" || password == "" {
        return "".to_string();
    }

    if username != &auth.username || password != &auth.password {
        return "".to_string();
    }

    let claims = Claims {
        sub: auth.username.to_owned(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 6 * 60 * 60,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth.secret.as_ref()),
    )
    .unwrap()
}

pub fn verify(auth: Arc<Auth>, req: HttpRequest) -> bool {
    let default_value = HeaderValue::from_str("").unwrap();
    let token = req
        .headers()
        .get("Auth")
        .unwrap_or(&default_value)
        .to_str()
        .unwrap_or("");

    if token == "" {
        return false;
    }

    let data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(auth.secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    let claims = match data {
        Ok(p) => p.claims,
        Err(_) => return false,
    };

    claims.sub == auth.username
}

pub fn check(auth: Arc<Auth>, req: HttpRequest) -> bool {
    !auth.enabled || verify(auth, req)
}
