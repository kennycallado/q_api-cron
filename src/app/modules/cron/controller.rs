use rocket::http::Status;

use rocket::State;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::services::claims::{Claims, UserInClaims};
use crate::app::providers::services::cron::CronManager;
use crate::app::providers::services::cron::CronJob as CronJobTrait;

use crate::app::modules::cron::model::{CronJob, NewCronJob};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        hello,
        hello_satete,
        add,
        index,
    ]
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello from cron"
}

#[get("/hello/state")]
pub async fn hello_satete() -> String {
    let res = reqwest::get("http://localhost:8000/api/v1/state/hello").await.unwrap();

    match res.text().await {
        Ok(text) => {
            text.to_owned()
        },
        Err(_) => "error".to_owned()
    }
}

#[post("/", data = "<new_cronjob>")]
pub async fn add(cron: &State<CronManager>, claims: AccessClaims, new_cronjob: Json<NewCronJob>) -> Result<Json<Uuid>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => helper(cron, claims.0.user, new_cronjob.into_inner()).await,
        _ => {
            println!("Error: add; Role not handled");
            Err(Status::Unauthorized)
        }
    }
}

async fn helper(cron: &State<CronManager>, _user: UserInClaims, new_cronjob: NewCronJob) -> Result<Json<Uuid>, Status> {
    let new_cronjob: NewCronJob = new_cronjob;
    let service = ConfigGetter::get_entity_url(new_cronjob.service.as_str()).unwrap();

    let job = Job::new_async(new_cronjob.schedule.as_str(), move |_uuid, _l|  {
        let url = format!("{}{}", service, new_cronjob.route);
        Box::pin(async move {
            let robo_token = Claims::from(UserInClaims::default()).enconde_for_robot();

            let res;
            {
                let client = reqwest::Client::new();
                res = client
                    .get(url)
                    .bearer_auth(robo_token.unwrap())
                    .header("Accept", "application/json")
                    .send().await.unwrap();
            }

            if let Err(e) = res.error_for_status_ref() {
                println!("Error: {}", e);
                println!("Maybe there is a way to inform and kill its own self")
            }
        })
    });

    match job {
        Ok(job) => {
            let id = job.guid();
            cron.add_job(job).await.unwrap();

            Ok(Json(id))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
pub async fn index(cron: &State<CronManager>, claims: AccessClaims) -> Result<Json<Vec<CronJobTrait>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => { let jobs = cron.get_jobs().await; Ok(Json(jobs)) },
        _ => {
            println!("Error: index; Role not handled");
            Err(Status::Unauthorized)
        }
    }
}
