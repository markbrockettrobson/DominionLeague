#[macro_use] extern crate rocket;

pub mod endpoints;
pub mod model;

use model::state::card_data::build_card_data;
use rocket::{Rocket, Build, build};

use crate::endpoints::{ 
    health::health, 
    card_json_from_id::card_json_from_id, 
    card_json_from_name::card_json_from_name };

#[launch]
#[mutants::skip]
fn launch_app() -> Rocket<Build> {
    build()
    .manage(build_card_data())
    .mount("/", routes![health, card_json_from_id, card_json_from_name])
}

#[cfg(test)]
mod test {
    use super::launch_app;

    #[test]
    fn launch_app_should_not_panic() {
        let _ = launch_app();
    }
}
