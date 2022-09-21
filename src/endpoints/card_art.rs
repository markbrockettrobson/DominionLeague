use rocket::{self, Rocket, Build, fs::{FileServer, relative}};

pub trait MountCardArtFileServer {
    fn mount_card_art_file_server(self) -> Rocket<Build>;
}

impl MountCardArtFileServer for Rocket<Build> {
    fn mount_card_art_file_server(self) -> Rocket<Build> {
        self.mount("/card", FileServer::from(relative!("src/model/scraped_data/cards")))
    }    
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::uri;

    use crate::launch_app;
    use crate::model::card::CardFilepath;
    use crate::model::data_loaders::card_data_loader::get_base_card_vec;

    #[test]
    fn test_card_art_from_file_name() {
        let base_cards = get_base_card_vec();
        let copper = &base_cards[0];

        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/Copper_1.jpeg")).dispatch();

        let file_result = File::open(copper.get_art_path(1));
        let file = match file_result {
            Ok(_file) => _file,
            Err(_) => panic!(),
        };

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        let _ = reader.read_to_end(&mut buffer);

        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_bytes().unwrap() == buffer);
    }

    #[test]
    fn test_card_art_from_file_name_unknowen_file_name() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/no_such_file_1.jpeg")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}