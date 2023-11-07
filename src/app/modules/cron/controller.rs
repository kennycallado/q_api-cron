use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::models::cronjob::PubNewCronJob;
use crate::app::providers::services::claims::{Claims, UserInClaims};
use crate::app::providers::services::cron::CronManager;
use crate::database::connection::Db;

use crate::app::modules::cron::model::{CronJob, CronJobComplete, NewCronJob, PostNewCronJob};
use crate::app::modules::cron::handlers::{index, show, create, delete};

use crate::app::modules::cron::services::repository as cron_repository;
use crate::app::modules::escalon::services::repository as escalon_repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,

        get_index,
        get_index_none,
        get_show,
        get_show_none,
        get_retry,
        get_retry_none,

        post_create,
        post_create_none,

        delete_remove,
        delete_remove_none,
    ]
}

#[options("/<_..>")]
pub fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: &Db, claims: AccessClaims) -> Result<Json<Vec<CronJob>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "coord" |
        "thera" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(db: &Db, claims: AccessClaims, id: i32) -> Result<Json<CronJobComplete>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "coord" |
        "thera" => show::get_show_admin(db, claims.0.user, id).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[allow(unused_variables)]
#[get("/<_id>", rank = 102)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_job>", rank = 1)]
pub async fn post_create(
    db: &Db,
    claims: AccessClaims,
    jm: &State<CronManager>,
    new_job: Json<PostNewCronJob>,
) -> Result<Json<CronJobComplete>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "coord" |
        "thera" => create::post_create_admin(db, claims.0.user, jm.inner(), new_job.into_inner()).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[post("/", rank = 2)]
pub async fn post_create_none() -> Status {
    Status::Unauthorized
}

#[delete("/<id>", rank = 101)]
pub async fn delete_remove(
    db: &Db,
    claims: AccessClaims,
    jm: &State<CronManager>,
    id: i32) -> Result<Json<CronJob>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "coord" |
        "thera" => delete::delete_remove_admin(db, claims.0.user, jm.inner(), id).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[allow(unused_variables)]
#[delete("/<id>", rank = 102)]
pub async fn delete_remove_none(id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>/retry", rank = 1)]
pub async fn get_retry(db: &Db, claims: AccessClaims, jm: &State<CronManager>, id: i32) -> Result<Json<CronJobComplete>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "coord" |
        "thera" => show::get_retry_admin(db, jm.inner(), claims.0.user, id).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[allow(unused_variables)]
#[get("/<id>/retry", rank = 2)]
pub async fn get_retry_none(id: i32) -> Status {
    Status::Unauthorized
}
