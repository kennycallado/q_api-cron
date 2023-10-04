use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::modules::cron::model::{CronJob, CronJobComplete};
use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::cron::services::repository as cron_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<CronJobComplete>, Status> {
    let job = cron_repository::get_complete(&db, id).await;

    match job {
        Ok(job) => Ok(Json(job.into())),
        Err(_) => Err(Status::NotFound),
    }
}
