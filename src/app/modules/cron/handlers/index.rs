use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::modules::cron::model::CronJob;
use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::cron::services::repository as cron_repository;

pub async fn get_index_admin(db: &Db, _admin: UserInClaims) -> Result<Json<Vec<CronJob>>, Status> {
    let jobs = cron_repository::get_all(&db).await;

    match jobs {
        Ok(jobs) => Ok(Json(jobs)),
        Err(_) => Err(Status::NotFound)
    }
}
