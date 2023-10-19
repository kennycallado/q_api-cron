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

pub async fn post_create_admin(
    db: &Db,
    _admin: UserInClaims,
    jm: &CronManager,
    post_job: PostNewCronJob,
) -> Result<Json<CronJobComplete>, Status> {
    let escalon = jm.inner().add_job(post_job.job.clone()).await;
    let job = match escalon_repository::insert(&db, escalon.clone().into()).await {
        Ok(job) => job,
        Err(e) => {
            println!(
                "Error: post_create_admin; escalon_repository::insert: {}",
                e
            );
            return Err(Status::InternalServerError);
        }
    };

    let new_job = NewCronJob {
        owner: ConfigGetter::get_identity(),
        service: post_job.service,
        route: post_job.route,
        job_id: escalon.job_id,
    };

    let cron_job = match cron_repository::insert(&db, new_job).await {
        Ok(cron_job) => cron_job,
        Err(e) => {
            println!("Error: post_create_admin; cron_repository::insert: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let job = CronJobComplete {
        id: cron_job.id,
        owner: cron_job.owner,
        service: cron_job.service,
        route: cron_job.route,
        job,
    };

    Ok(Json(job))
}
