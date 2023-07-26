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

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        add,
        delete,
    ]
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
    let new_cronjob: NewCronJob = new_cronjob; // ???
    // let route = new_cronjob.route.clone();
    // let service = ConfigGetter::get_entity_url(new_cronjob.service.as_str()).unwrap();
    // let manager: CronManager = cron.inner().clone();

    let new_cronjob = PubNewCronJob {
        route: new_cronjob.route,
        schedule: new_cronjob.schedule,
        service: new_cronjob.service,
        status: "pending".to_owned(),
        until: new_cronjob.until,
        since: new_cronjob.since,
    };

    match cron.create_job(&new_cronjob).await {
        Ok(id) => Ok(Json(id)),
        Err(e) => {
            println!("Error: {};", e);
            Err(Status::InternalServerError)
        }
    }

    // all inside the closure is executed every time
    // let job = Job::new_async(new_cronjob.schedule.as_str(), move |uuid, mut lock|  {
    //     let url = format!("{}{}", service, route);
    //     let manager = manager.clone();

    //     Box::pin(async move {
    //         let job = manager.get_job(uuid).await.unwrap();
    //         let next_tick = lock.next_tick_for_job(uuid).await.unwrap().unwrap().naive_utc();
    //         let until = job.until.clone();

    //         // println!("Job {} executed at {:?}", uuid, next_tick);
    //         // println!("Until: {:?}", until);
    //         // println!("Difference: {:?}", until - next_tick);

    //         // first layer to match state of job
    //         // if job is finished or error, remove from scheduler
    //         // if job is active, do nothing
    //         // if job is pending, execute
    //         // this way we can handle the state of the job
    //         // and remove it from the scheduler when it's finished
    //         // or when it's failed

    //         // println!("Status: {}", job.status);

    //         manager.status_hanler(&mut lock, job, next_tick).await;

    //         if let Some(until) = until {
    //             if until < next_tick {
    //                 // change state to finished
    //                 manager.update_status(uuid, "finished").await.unwrap();
    //                 // lock.remove(&uuid).await.unwrap();

    //                 return ;
    //             }
    //         }

    //         let robo_token = Claims::from(UserInClaims::default()).enconde_for_robot();
    //         let res;
    //         {
    //             let client = reqwest::Client::new();
    //             res = client
    //                 .get(url)
    //                 .bearer_auth(robo_token.unwrap())
    //                 .header("Accept", "application/json")
    //                 .send().await;
    //         }

    //         if let Err(e) = res {
    //             println!("Error: {};", e);

    //             // change the state to error
    //             manager.update_status(uuid, "error").await.unwrap();
    //             // lock.remove(&uuid).await.unwrap();
    //        }
    //     })
    // });

    // Only executed on the first time
    // match job {
    //     Ok(job) => {
    //         let id = job.guid();

    //         let cron_job = PubNewCronJob {
    //             schedule: new_cronjob.schedule,
    //             service: new_cronjob.service,
    //             route: new_cronjob.route,
    //             since: new_cronjob.since,
    //             until: new_cronjob.until,
    //         };

    //         cron.add_job(job, cron_job).await.unwrap();

    //         Ok(Json(id))
    //     },
    //     Err(e) => {
    //         println!("Error: {};", e);
    //         Err(Status::InternalServerError)
    //     },
    // }
}

#[get("/")]
pub async fn index(cron: &State<CronManager>, claims: AccessClaims) -> Result<Json<Vec<PubCronJob>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => { let jobs = cron.get_jobs().await; Ok(Json(jobs)) },
        _ => {
            println!("Error: index; Role not handled");
            Err(Status::Unauthorized)
        }
    }
}

#[delete("/<id>")]
pub async fn delete(cron: &State<CronManager>, claims: AccessClaims, id: Uuid) -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => {
            match cron.remove_job(id).await {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        _ => {
            println!("Error: delete; Role not handled");
            Err(Status::Unauthorized)
        }
    }
}
