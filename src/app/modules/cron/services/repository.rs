use diesel::prelude::*;
use rocket::serde::uuid::Uuid;

use crate::app::modules::cron::model::{CronJob, NewCronJob, CronJobComplete};
use crate::app::modules::escalon::model::EJob;
use crate::database::connection::Db;
use crate::database::schema::{cronjobs, escalonjobs};

pub async fn get_all(db: &Db) -> Result<Vec<CronJob>, diesel::result::Error> {
    db.run(move |conn| {
        cronjobs::table
            .load::<CronJob>(conn)
    }).await
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<CronJob, diesel::result::Error> {
    db.run(move |conn| {
        cronjobs::table
            .find(id)
            .first::<CronJob>(conn)
    }).await
}

pub async fn get_by_job_id(db: &Db, job_id: Uuid) -> Result<CronJob, diesel::result::Error> {
    db.run(move |conn| {
        cronjobs::table
            .filter(cronjobs::job_id.eq(job_id))
            .first::<CronJob>(conn)
    }).await
}

pub async fn get_complete(db: &Db, id: i32) -> Result<CronJobComplete, diesel::result::Error> {
    db.run(move |conn| {
        let app_job = cronjobs::table
            .find(id)
            .first::<CronJob>(conn)?;

        let escalon_job = escalonjobs::table
            .find(app_job.job_id)
            .first::<EJob>(conn)?;

        Ok(CronJobComplete {
            id: app_job.id,
            owner: app_job.owner,
            service: app_job.service,
            route: app_job.route,
            job: escalon_job,
        })
    }).await
}

pub async fn create(db: &Db, app_job: NewCronJob) -> Result<CronJobComplete, diesel::result::Error> {
    db.run(move |conn| {
        let app_job = diesel::insert_into(cronjobs::table)
            .values(app_job)
            .get_result::<CronJob>(conn)?;

        let escalon_job = escalonjobs::table
            .find(app_job.job_id)
            .first::<EJob>(conn)?;

        Ok(CronJobComplete {
            id: app_job.id,
            owner: app_job.owner,
            service: app_job.service,
            route: app_job.route,
            job: escalon_job,
        })
    }).await
}
