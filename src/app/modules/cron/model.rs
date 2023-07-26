use chrono::{DateTime, Utc, NaiveDateTime};
use rocket::serde::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::app::providers::models::cronjob::PubCronJob;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CronJob {
    pub id: Uuid,
    pub schedule: String,
    pub service: String,
    pub status: String,
    pub route: String,
    pub since: Option<NaiveDateTime>,
    pub until: Option<NaiveDateTime>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCronJob {
    pub schedule: String,
    pub service: String,
    pub route: String,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>
}
