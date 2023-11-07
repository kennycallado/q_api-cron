use rocket::serde::uuid::Uuid;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::cron::model::{CronJob, CronJobComplete, NewCronJob};
use crate::app::modules::escalon::model::EJob;
use crate::database::connection::Db;

pub async fn get_all(db: &Db) -> Result<Vec<CronJob>, sqlx::Error> {
    sqlx::query_as!(CronJob, "SELECT * FROM cronjobs")
        .fetch_all(&db.0)
        .await
    // db.run(move |conn| cronjobs::table.load::<CronJob>(conn))
    //     .await
}

// pub async fn get_by_id(db: &Db, id: i32) -> Result<CronJob, sqlx::Error> {
//     db.run(move |conn| cronjobs::table.find(id).first::<CronJob>(conn))
//         .await
// }

// pub async fn get_by_job_id(db: &Db, job_id: Uuid) -> Result<CronJob, sqlx::Error> {
//     db.run(move |conn| {
//         cronjobs::table
//             .filter(cronjobs::job_id.eq(job_id))
//             .first::<CronJob>(conn)
//     })
//     .await
// }

pub async fn get_complete(db: &Db, id: i32) -> Result<CronJobComplete, sqlx::Error> {
    let app_job = sqlx::query_as!(CronJob, "SELECT * FROM cronjobs WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;

    let escalon_job = sqlx::query_as!(EJob, "SELECT * FROM escalonjobs WHERE id = $1", app_job.job_id).fetch_one(&db.0).await?;

    Ok(CronJobComplete {
        id: app_job.id,
        owner: app_job.owner,
        service: app_job.service,
        route: app_job.route,
        job: escalon_job,
    })

    // db.run(move |conn| {
    //     let app_job = cronjobs::table.find(id).first::<CronJob>(conn)?;

    //     let escalon_job = escalonjobs::table
    //         .find(app_job.job_id)
    //         .first::<EJob>(conn)?;

    //     Ok(CronJobComplete {
    //         id: app_job.id,
    //         owner: app_job.owner,
    //         service: app_job.service,
    //         route: app_job.route,
    //         job: escalon_job,
    //     })
    // })
    // .await
}

pub async fn insert(db: &Db, app_job: NewCronJob) -> Result<CronJob, sqlx::Error> {
    sqlx::query_as!(
        CronJob,
        "INSERT INTO cronjobs (owner, service, route, job_id) VALUES ($1, $2, $3, $4) RETURNING *",
        app_job.owner,
        app_job.service,
        app_job.route,
        app_job.job_id
    ).fetch_one(&db.0).await

    // db.run(move |conn| {
    //     diesel::insert_into(cronjobs::table)
    //         .values(app_job)
    //         .get_result::<CronJob>(conn)
    // })
    // .await
}

pub async fn update(db: &Db, id: i32, app_job: NewCronJob) -> Result<CronJob, sqlx::Error> {
    sqlx::query_as!(
        CronJob,
        "UPDATE cronjobs SET owner = $1, service = $2, route = $3, job_id = $4 WHERE id = $5 RETURNING *",
        app_job.owner,
        app_job.service,
        app_job.route,
        app_job.job_id,
        id
    ).fetch_one(&db.0).await

    // db.run(move |conn| {
    //     diesel::update(cronjobs::table.find(id))
    //         .set(&app_job)
    //         .get_result::<CronJob>(conn)
    // }).await
}

pub async fn delete(db: &Db, id: i32) -> Result<CronJob, sqlx::Error> {
    sqlx::query_as!(CronJob, "DELETE FROM cronjobs WHERE id = $1 RETURNING *", id)
        .fetch_one(&db.0)
        .await

    // db.run(move |conn| {
    //     diesel::delete(cronjobs::table.find(id)).get_result::<CronJob>(conn)
    // })
    // .await
}
