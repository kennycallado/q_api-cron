use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::modules::cron::model::{CronJob, CronJobComplete, NewCronJob};
use crate::app::modules::escalon::model::NewEJob;
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::cron::CronManager;
use crate::database::connection::Db;

use crate::app::modules::cron::services::repository as cron_repository;
use crate::app::modules::escalon::services::repository as escalon_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<CronJobComplete>, Status> {
    let job = cron_repository::get_complete(&db, id).await;

    match job {
        Ok(job) => Ok(Json(job.into())),
        Err(_) => Err(Status::NotFound),
    }
}

pub async fn get_retry_admin(db: &Db, jm: &CronManager, _admin: UserInClaims, id: i32) -> Result<Json<CronJobComplete>, Status> {
    let old_job = cron_repository::get_complete(&db, id).await;

    match old_job {
        Ok(old_job) => {
            let old_ejob = old_job.job.clone();
            let new_ejob: NewEJob = old_job.job.clone().into();

            let escalon = jm.inner().add_job(new_ejob).await;
            let job = match escalon_repository::insert(&db, escalon.clone().into()).await {
                Ok(job) => job,
                Err(e) => {
                    println!("Error: get_retry_admin; escalon_repository::insert: {}", e);
                    return Err(Status::InternalServerError);
                }
            };
            
            let new_job = NewCronJob {
                owner: ConfigGetter::get_identity(),
                service: old_job.service,
                route: old_job.route,
                job_id: escalon.job_id,
            };

            // update cronjob
            let cron_job = match cron_repository::update(db, id, new_job).await {
                Ok(cron_job) => cron_job,
                Err(e) => {
                    println!("Error: get_retry_admin; cron_repository::update {}", e);
                    return Err(Status::InternalServerError);
                }
            };

            // remove old_ejob careful because CASCADE
            escalon_repository::delete(db, old_ejob.id).await.unwrap();

            let job = CronJobComplete {
                id: cron_job.id,
                owner: cron_job.owner,
                service: cron_job.service,
                route: cron_job.route,
                job,
            };

            Ok(Json(job.into()))
        },
        Err(_) => Err(Status::NotFound),
    }
}
