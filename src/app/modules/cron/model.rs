use rocket::serde::uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CronJob {
    pub id: Uuid,
    pub schedule: String,
    pub route: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCronJob {
    pub schedule: String,
    pub service: String,
    pub route: String,
}
