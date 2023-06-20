use crate::app::modules::cron::controller::routes as cron_routes;
use crate::app::modules::state::controller::routes as state_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async { 
        rocket
            .mount("/api/v1/cron", cron_routes())
            .mount("/api/v1/state", state_routes())
        })
}
