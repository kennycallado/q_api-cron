#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubRecord {
    pub id: i32,
    pub user_id: i32,
    pub record: rocket::serde::json::Value,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubNewRecord {
    pub user_id: i32,
<<<<<<< HEAD
    pub record: rocket::serde::json::Value,
=======
    pub record: Option<rocket::serde::json::Value>,
>>>>>>> 0b9aa827104d7b7a68bdca13745b0562a965efc0
}
