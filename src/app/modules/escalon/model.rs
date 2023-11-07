use chrono::{DateTime, Utc, NaiveDateTime};
use escalon_jobs::EscalonJobStatus;
use escalon_jobs::{EscalonJob, EscalonJobTrait, NewEscalonJob};
use rocket::serde::uuid::Uuid;
use serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, sqlx::FromRow};

use crate::app::modules::cron::model::CronJob;
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::cronjob::PubEJob;
use crate::app::providers::services::claims::{Claims, UserInClaims};
use crate::app::providers::services::cron::Context;
use crate::database::connection::Db;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct EJob {
    pub id: Uuid,
    pub status: String,
    pub schedule: String,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

impl From<EscalonJob> for EJob {
    fn from(escalon: EscalonJob) -> Self {
        Self {
            id: escalon.job_id,
            status: match escalon.status {
                EscalonJobStatus::Scheduled => "scheduled".to_string(),
                EscalonJobStatus::Running => "running".to_string(),
                EscalonJobStatus::Done => "done".to_string(),
                EscalonJobStatus::Failed => "failed".to_string(),
            },
            schedule: escalon.schedule,
            // since: escalon.since.map(|d| DateTime::from_utc(d, Utc)),
            // until: escalon.until.map(|d| DateTime::from_utc(d, Utc)),
            since: escalon.since.map(|d| DateTime::from_naive_utc_and_offset(d, Utc)),
            until: escalon.until.map(|d| DateTime::from_naive_utc_and_offset(d, Utc)),
        }
    }
}

impl From<EJob> for NewEJob {
    fn from(ejob: EJob) -> Self {
        Self {
            schedule: ejob.schedule,
            since: ejob.since,
            until: ejob.until,
        }
    }
}

impl From<EJob> for PubEJob {
    fn from(ejob: EJob) -> Self {
        let since = ejob.since.map(|d| NaiveDateTime::from_timestamp_opt(d.timestamp(), 0));
        let until = ejob.until.map(|d| NaiveDateTime::from_timestamp_opt(d.timestamp(), 0));

        PubEJob {
            id: ejob.id,
            status: ejob.status,
            schedule: ejob.schedule,
            since: since.unwrap(),
            until: until.unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewEJob {
    pub schedule: String,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

impl From<NewEJob> for NewEscalonJob {
    fn from(ejob: NewEJob) -> Self {
        NewEscalonJob {
            schedule: ejob.schedule,
            since: ejob.since.map(|d| d.naive_utc()),
            until: ejob.until.map(|d| d.naive_utc()),
        }
    }
}

#[rocket::async_trait]
impl EscalonJobTrait<Context> for NewEJob {
    async fn run_job(&self, context: &Context, mut job: EscalonJob) -> EscalonJob {
        let cron_job = match sqlx::query_as!(CronJob, "SELECT * FROM cronjobs WHERE job_id = $1", job.job_id).fetch_one(&context.db).await {
            Ok(cron_job) => cron_job,
            Err(e) => {
                println!("Err: {}", e);
                job.status = EscalonJobStatus::Failed;
                return job;
            }
        };

        let robo_token = Claims::from(UserInClaims::default()).enconde_for_robot();

        // WARNING: This is a hack, the service should be a url
        // let service = cron_job.service;
        let service = ConfigGetter::get_entity_url(&cron_job.service).unwrap();
        let url = format!("{}{}", service, cron_job.route);

        let res = context
            .fetch
            .get(url)
            .bearer_auth(robo_token.unwrap())
            .header("Accept", "application/json")
            .send()
            .await;

        match res {
            Ok(res) => {
                if !res.status().is_success() {
                    job.status = EscalonJobStatus::Failed;
                    println!("No success");
                }

                job
            }
            Err(e) => {
                println!("Err: Failed");
                println!("Err: {}", e);
                job.status = EscalonJobStatus::Failed;

                job
            }
        }
    }
}
