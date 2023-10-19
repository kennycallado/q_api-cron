use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::modules::cron::model::{CronJob, CronJobComplete, NewCronJob, PostNewCronJob};
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::cronjob::PubCronJob;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::cron::CronManager;
use crate::database::connection::Db;

use crate::app::modules::cron::services::repository as cron_repository;
use crate::app::modules::escalon::services::repository as escalon_repository;

pub async fn delete_remove_admin(
    db: &Db,
    _admin: UserInClaims,
    jm: &CronManager,
    id: i32,
) -> Result<Json<CronJob>, Status> {
    let job = match cron_repository::delete(&db, id).await {
        Ok(job) => job,
        Err(_) => return Err(Status::BadRequest),
    };

    let ejob = match escalon_repository::delete(&db, job.job_id).await {
        Ok(job) => job,
        Err(_) => return Err(Status::BadRequest),
    };

    jm.inner().remove_job(ejob.id).await;

    Ok(Json(job))
}
