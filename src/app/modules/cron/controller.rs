use rocket::http::Status;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::app::modules::cron::model::{CronJob, CronJobComplete, NewCronJob, PostNewCronJob};
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::models::cronjob::{PubCronJob, PubNewCronJob};
use crate::app::providers::services::claims::{Claims, UserInClaims};
use crate::app::providers::services::cron::CronManager;
use crate::database::connection::Db;

use crate::app::modules::cron::services::repository as cron_repository;
use crate::app::modules::escalon::services::repository as escalon_repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![index, show, create]
}

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<CronJob>> {
    let jobs = cron_repository::get_all(&db).await.unwrap();

    Json(jobs)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<PubCronJob> {
    let job = cron_repository::get_complete(&db, id).await.unwrap();

    Json(job.into())
}

// pub async fn create(db: Db, jm: &State<EscalonJobsManager<Context<ConnectionPool<Db, PgConnection>>>>, new_job: Json<PostNewCronJob>) -> Json<CronJobComplete> {
//
#[post("/", data = "<new_job>")]
pub async fn create(
    db: Db,
    jm: &State<CronManager>,
    new_job: Json<PostNewCronJob>,
) -> Json<CronJobComplete> {
    let post_job = new_job.into_inner();

    let escalon_job = jm.inner().inner().add_job(post_job.job.clone()).await;
    escalon_repository::insert(&db, escalon_job.clone().into())
        .await
        .unwrap();

    let new_job = NewCronJob {
        owner: ConfigGetter::get_identity(),
        service: post_job.service,
        route: post_job.route,
        job_id: escalon_job.job_id.clone(),
    };

    let job = cron_repository::create(&db, new_job).await.unwrap();

    Json(job.into())
}
