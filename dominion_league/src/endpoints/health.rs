use rocket::{self, get};

#[get("/health")]
pub fn health() -> &'static str {
    "Ok"
}

#[cfg(test)]
mod test {    
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::uri;

    use crate::launch_app;

    #[test]
    fn health() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!(super::health)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Ok");
    }
}