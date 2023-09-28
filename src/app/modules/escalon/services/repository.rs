use diesel::prelude::*;
use rocket::serde::uuid::Uuid;

use crate::app::modules::escalon::model::EJob;
use crate::database::connection::Db;
use crate::database::schema::escalonjobs;

pub async fn insert(db: &Db, escalon_job: EJob) -> Result<EJob, diesel::result::Error> {
    db.run(move |conn| {
        diesel::insert_into(escalonjobs::table)
            .values(escalon_job)
            .get_result::<EJob>(conn)
    })
    .await
}
