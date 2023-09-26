use chrono::{DateTime, Utc, NaiveDateTime};
use rocket::serde::uuid::Uuid;
use serde::{Deserialize, Serialize};
#[cfg(all(feature = "db", feature = "cron"))]
use diesel::PgConnection;

#[cfg(all(feature = "db", feature = "cron"))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubCronJob {
    pub id: i32,
    pub owner: String,
    pub service: String,
    pub route: String,
    pub job: PubEJob,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubEJob {
    pub id: Uuid,
    pub status: String,
    pub schedule: String,
    pub since: Option<NaiveDateTime>,
    pub until: Option<NaiveDateTime>,
}

#[cfg(not(feature = "db"))]
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[cfg(all(feature = "db", feature = "cron"))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubNewCronJob {
    pub service: String,
    pub route: String,
    pub job: NewEJob,
}


#[cfg(all(feature = "db", feature = "cron"))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewEJob {
    pub schedule: String,
    pub since: Option<NaiveDateTime>,
    pub until: Option<NaiveDateTime>,
}

#[cfg(not(feature = "db"))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubNewCronJob {
    pub schedule: String,
    pub service: String,
    pub status: String,
    pub route: String,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>
}

#[cfg(all(feature = "db", feature = "cron"))]
impl From<PubCronJob> for PubNewCronJob {
    fn from(cronjob: PubCronJob) -> Self {
        PubNewCronJob {
            service: cronjob.service,
            route: cronjob.route,
            job: NewEJob {
                schedule: cronjob.job.schedule,
                since: cronjob.job.since,
                until: cronjob.job.until,
            }
        }
    }
}

// #[cfg(all(feature = "db", feature = "cron"))]
// pub struct DbCron(pub PgConnection);
