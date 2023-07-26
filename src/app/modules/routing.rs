use crate::app::modules::cron::controller::routes as cron_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async { 
        rocket
            .mount("/api/v1/cron", cron_routes())
        })
}
