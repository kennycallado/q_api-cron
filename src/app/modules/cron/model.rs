use chrono::{DateTime, Utc};
use rocket::serde::uuid::Uuid;
use serde::{Deserialize, Serialize};

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::escalon::model::{EJob, NewEJob};
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::cronjob::PubCronJob;

#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CronJob {
    pub id: i32,
    pub owner: String,
    pub service: String,
    pub route: String,
    pub job_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCronJob {
    pub owner: String,
    pub service: String,
    pub route: String,
    pub job_id: Uuid,
}

impl From<CronJob> for NewCronJob {
    fn from(cronjob: CronJob) -> Self {
        NewCronJob {
            owner: cronjob.owner,
            service: cronjob.service,
            route: cronjob.route,
            job_id: cronjob.job_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostNewCronJob {
    pub service: String,
    pub route: String,
    pub job: NewEJob,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CronJobComplete {
    pub id: i32,
    pub owner: String,
    pub service: String,
    pub route: String,
    pub job: EJob,
}

impl From<CronJobComplete> for PubCronJob {
    fn from(cronjob: CronJobComplete) -> Self {
        PubCronJob {
            id: cronjob.id,
            owner: cronjob.owner,
            service: cronjob.service,
            route: cronjob.route,
            job: cronjob.job.into(),
        }
    }
}
