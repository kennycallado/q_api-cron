use rocket::serde::uuid::Uuid;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::escalon::model::EJob;
use crate::database::connection::Db;

pub async fn insert(db: &Db, escalon_job: EJob) -> Result<EJob, sqlx::Error> {
    sqlx::query_as!(
        EJob,
        "INSERT INTO escalonjobs (id, status, schedule, since, until) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        escalon_job.id,
        escalon_job.status,
        escalon_job.schedule,
        escalon_job.since,
        escalon_job.until
    ).fetch_one(&db.0).await

    // db.run(move |conn| {
    //     diesel::insert_into(escalonjobs::table)
    //         .values(escalon_job)
    //         .get_result::<EJob>(conn)
    // })
    // .await
}

pub async fn delete(db: &Db, id: Uuid) -> Result<EJob, sqlx::Error> {
    sqlx::query_as!(
        EJob,
        "DELETE FROM escalonjobs WHERE id = $1 RETURNING *",
        id
    ).fetch_one(&db.0).await

    // db.run(move |conn| {
    //     diesel::delete(escalonjobs::table.find(id)).get_result::<EJob>(conn)
    // })
    // .await
}
