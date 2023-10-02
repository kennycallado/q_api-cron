use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::PgConnection;
use escalon_jobs::EscalonJobStatus;
use escalon_jobs::{EscalonJob, EscalonJobTrait, NewEscalonJob};
use rocket::serde::uuid::Uuid;
use rocket_sync_db_pools::ConnectionPool;
use serde::{Deserialize, Serialize};

use crate::app::modules::cron::model::CronJob;
use crate::app::providers::models::cronjob::PubEJob;
use crate::app::providers::services::cron::{Context, ContextDb};
use crate::database::connection::Db;
use crate::database::schema::escalonjobs;

// use crate::app::server::Context;
// use crate::app::modules::cron::model::{NewAppJob, AppJob, AppJobComplete};

#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset,
)]
#[diesel(table_name = escalonjobs)]
#[serde(crate = "rocket::serde")]
pub struct EJob {
    pub id: Uuid,
    pub status: String,
    pub schedule: String,
    pub since: Option<NaiveDateTime>,
    pub until: Option<NaiveDateTime>,
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
            since: escalon.since,
            until: escalon.until,
        }
    }
}

impl From<EJob> for NewEJob {
    fn from(ejob: EJob) -> Self {
        Self {
            schedule: ejob.schedule,
            since: ejob
                .since
                .map(|d| DateTime::from_naive_utc_and_offset(d, Utc)),
            until: ejob
                .until
                .map(|d| DateTime::from_naive_utc_and_offset(d, Utc)),
        }
    }
}

impl From<EJob> for PubEJob {
    fn from(ejob: EJob) -> Self {
        PubEJob {
            id: ejob.id,
            status: ejob.status,
            schedule: ejob.schedule,
            since: ejob.since,
            until: ejob.until,
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
impl EscalonJobTrait<Context<ContextDb>> for NewEJob {
    async fn run_job(&self, context: &Context<ContextDb>, mut job: EscalonJob) -> EscalonJob {
        use crate::database::schema::{cronjobs, escalonjobs};
        use diesel::prelude::*;

        let cron_job: CronJob = context
            .db_pool
            .get()
            .await
            .unwrap()
            .run(move |conn| {
                let cron_job = cronjobs::table
                    .filter(cronjobs::job_id.eq(job.job_id))
                    .first::<CronJob>(conn);

                match cron_job {
                    Ok(cron_job) => cron_job,
                    Err(_) => panic!("Cron job not found"),
                }
            })
            .await;

        let robo_token = Claims::from(UserInClaims::default()).enconde_for_robot();
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
