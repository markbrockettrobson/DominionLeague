#[macro_use] extern crate rocket;

pub mod endpoints;
pub mod model;

use rocket::{Rocket, Build, build};

use crate::endpoints::health::health;

#[launch]
#[mutants::skip]
fn launch_app() -> Rocket<Build> {
    build().mount("/", routes![health])
}

#[cfg(test)]
mod test {
    use super::launch_app;

    #[test]
    fn launch_app_should_not_panic() {
        let _ = launch_app();
    }
}
