use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::PgConnection;
use rocket::serde::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::database::schema::cronjobs;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = cronjobs)]
#[serde(crate = "rocket::serde")]
pub struct PubCronJob {
    pub id: Uuid,
    pub schedule: String,
    pub service: String,
    pub status: String,
    pub route: String,
    pub since: Option<NaiveDateTime>,
    pub until: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = cronjobs)]
#[serde(crate = "rocket::serde")]
pub struct PubNewCronJob {
    pub schedule: String,
    pub service: String,
    pub status: String,
    pub route: String,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>
}

impl From<PubCronJob> for PubNewCronJob {
    fn from(cronjob: PubCronJob) -> Self {
        PubNewCronJob {
            schedule: cronjob.schedule,
            service: cronjob.service,
            status: cronjob.status,
            route: cronjob.route,
            since: cronjob.since.map(|d| DateTime::from_utc(d, Utc)),
            until: cronjob.until.map(|d| DateTime::from_utc(d, Utc)),
        }
    }
}

pub struct DbCron(pub PgConnection);
