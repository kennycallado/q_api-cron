use rocket::http::Status;

use rocket::State;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::models::cronjob::{PubNewCronJob, PubCronJob};
use crate::app::providers::services::claims::{Claims, UserInClaims};
use crate::app::providers::services::cron::CronManager;
use crate::app::modules::cron::model::{CronJob, NewCronJob};
use crate::database::connection::Db;

use crate::app::modules::cron::services::repository as cron_repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![ index, show,
        create
    ]
}

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<CronJob>> {
    let jobs = cron_repository::get_all(&db).await.unwrap();

    Json(jobs)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<PubCronJob> {
    let job = cron_repository::get_complete(&db, id).await.unwrap();

    Json(job)
}

#[post("/", data = "<new_job>")]
pub async fn create(db: Db, jm: &State<EscalonJobsManager<Context<ConnectionPool<Db, PgConnection>>>>, new_job: Json<PostNewCronJob>) -> Json<CronJobComplete> {
    let new_job = new_job.into_inner();

    // let escalon_job = jm.inner().0.escalon.add_job(new_job.job.clone()).await;
    let escalon_job = jm.inner().add_job(new_job.job.clone()).await;
    let new_job = NewCronJob {
        owner: ConfigGetter::get_identity(),
        service: new_job.service,
        route: new_job.route,
        job_id: escalon_job.job_id.clone(),
    };

    escalon_repository::insert(&db, escalon_job.into()).await.unwrap();

    let job = cron_repository::create(&db, new_job).await.unwrap();
    let job = cron_repository::get_complete(&db, job.id).await.unwrap();


    println!("Added");
    println!("Current jobs: {:?}", jm.jobs.lock().unwrap().len());

    Json(job)
}
