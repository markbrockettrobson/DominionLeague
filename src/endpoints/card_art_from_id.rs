use rocket::{self, State, response::Redirect};

use crate::model::{state::card_data::CardData, card::CardFilepath};


#[get("/card/<id>", rank=2)]
pub fn card_art_from_id(card_data: &State<CardData>,  id: u16) -> Redirect {
    if card_data.id_card_map.contains_key(&id) {
        let card = &card_data.id_card_map[&id];

        let edition = match card.editions.last() {
            Some(edition) => edition,
            None => &u8::MAX,
        };
        let card_file_name = format!("../../card/{}.jpeg",&card_data.id_card_map[&id].get_art_file_name(*edition));
        return Redirect::to(card_file_name);
    }
    Redirect::to("../../card/no_such_card_id")
}

#[get("/card/<id>/<edition>", rank=3)]
pub fn card_art_from_id_with_edition(card_data: &State<CardData>,  id: u16, edition: u8) -> Redirect {
    if card_data.id_card_map.contains_key(&id) {
        let card = &card_data.id_card_map[&id];
        if card.editions.contains(&edition){ 
            let card_file_name = format!("../../card/{}.jpeg",card.get_art_file_name(edition));
            return Redirect::to(card_file_name);
        }        
        return Redirect::to("../../card/no_such_edition"); 
    }
    Redirect::to("../../card/no_such_card_id")
}


#[cfg(test)]
mod test {

    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::uri;

    use crate::launch_app;

    #[test]
    fn test_card_art_from_id() {

        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/0")).dispatch();

        let headers_location = response.headers().get("Location").next();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(headers_location, Some("../../card/Copper_2.jpeg"));
    }

    #[test]
    fn test_card_art_from_id_no_id() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/65500")).dispatch();

        let headers_location = response.headers().get("Location").next();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(headers_location, Some("../../card/no_such_card_id"));
    }

    #[test]
    fn test_card_art_from_id_edition() {

        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/0/1")).dispatch();

        let headers_location = response.headers().get("Location").next();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(headers_location, Some("../../card/Copper_1.jpeg"));
    }

    #[test]
    fn test_card_art_from_id_no_id_edition() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/65500/1")).dispatch();

        let headers_location = response.headers().get("Location").next();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(headers_location, Some("../../card/no_such_card_id"));
    }

    #[test]
    fn test_card_art_from_id_unknowen_edition() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!("/card/0/3")).dispatch();
        
        let headers_location = response.headers().get("Location").next();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(headers_location, Some("../../card/no_such_edition"));
    }
}