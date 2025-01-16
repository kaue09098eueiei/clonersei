#![allow(unused_imports)]

mod routes;

#[macro_use]
extern crate rocket;

use mysql::Pool;
use std::any::Any;
use std::sync::Arc;
use serde_json::{json, Value};
use rocket::{Request, State};
use mysql::prelude::Queryable;
use rocket::serde::json::Json;
use std::collections::{BTreeMap, LinkedList};
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;
use regex::Regex;
use rocket::tokio::sync::Mutex;
use rocket::form::validate::Contains;
use rocket::fs::Options;
use rocket::http::{ContentType, Method, Status};
use rocket::serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub struct Authorization(String);

#[derive(Debug)]
pub enum AuthorizationError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) ->  Outcome<Self, Self::Error> {
        let token = req.headers().get_one("Authorization");
        match token {
            Some(token) => {
                Outcome::Success(Authorization(token.to_string()))
            }
            None => Outcome::Error((Status::Unauthorized, AuthorizationError::Missing)),
        }
    }
}

pub struct IP(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IP {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) ->  Outcome<Self, Self::Error> {
        match req.headers().get_one("X-Real-IP")
            .or_else(|| req.headers().get_one("CF-Connecting-IP"))
            .or_else(|| req.headers().get_one("X-Forwarded-For")) {
            Some(ip) => Outcome::Success(IP(ip.to_string())),
            None => {
                match req.client_ip() {
                    Some(ip) => Outcome::Success(IP(ip.to_string())),
                    None => Outcome::Success(IP("Unknown".to_string())),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    creators: Vec<String>,
    products: Vec<Product>,
    vouchers: Vec<Voucher>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: u32,
    name: String,
    price: f32,
    image: String,
    #[serde(rename = "type")]
    product_type: u32,
    item_id: Option<u32>,
    invisible: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Voucher {
    id: u32,
    name: String,
    discount: u32,
    #[serde(rename = "allowedProducts")]
    allowed_products: Vec<u32>,
}

pub fn is_valid_nick(nick: &str) -> bool {
    let re = match Regex::new(r"^[a-zA-Z0-9!@#$%^&*()_+\[\];\\<>,./?`~]{5,30}$") {
        Ok(re) => re,
        Err(_) => return false,
    };

    re.is_match(nick)
}

#[launch]
fn rocket() -> _ {
    let codes = Arc::new(Mutex::new(BTreeMap::<String, (String, u64)>::new()));
    let pool = Arc::new(Mutex::new(Pool::new("mysql://root:kNXcBe%238Jd!XexGrT%267YmAtt5Lr%23hgt%24%24ebKS!%24Q@127.0.0.1:3306/database")
        .expect("Failed to connect to database")));

    let gifts: Arc<Mutex<LinkedList<String>>> = Arc::new(Mutex::new(LinkedList::new()));
    let file = File::open("settings.json").expect("Failed to open settings.json");

    let settings: Settings = serde_json::from_reader(file).expect("Failed to parse settings.json");

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Options, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .manage(pool)
        .manage(codes)
        .manage(settings)
        .manage(gifts)
        .mount("/", routes![
            routes::index::get_index,
            routes::bans::post_ban,
            routes::bans::get_bans,
            routes::players::get_players,
            routes::players::post_player,
            routes::sync::get_sync,
            routes::sync::get_connections,
            routes::activation::post_activation,
            routes::discord::post_check,
            routes::discord::get_discord,
            routes::webhook::post_webhook,
            routes::products::get_products,
            routes::creators::get_creators,
            routes::payments::post_checkout,
            routes::products::get_voucher,
            routes::payments::post_giftcard,
        ])
}
