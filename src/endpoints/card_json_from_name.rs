use rocket::{self, get, State, serde::json::serde_json};

use crate::model::state::card_data::CardData;


#[get("/card_json/<name>", rank=2)]
pub fn card_json_from_name(card_data: &State<CardData>,  name: String) -> String {
    if card_data.name_card_map.contains_key(&name) {
       return serde_json::to_string(&card_data.name_card_map[&name]).unwrap();
    }
    "Unknown card name".to_string()
}

#[cfg(test)]
mod test {    
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::uri;

    use crate::launch_app;

    #[test]
    fn test_card_json_from_name() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!(super::card_json_from_name("Chapel".to_string()))).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), 
            "{\"id\":101,\"name\":\"Chapel\",\"supply_card\":true,\"basic_card\":false,\"card_counts\":[10,10,10,10,10],\"set_id\":1,\"editions\":[1,2],\"card_tags\":[\"IsAction\",\"Costs2\",\"NetLoseCards\",\"TrashFromHand\",\"TrashMultipleFromHand\",\"TrashTreasre\",\"TrashAction\",\"TrashVictory\",\"TrashCurse\"],\"kingdom_requirements\":[],\"kingdom_synergies\":[],\"kingdom_anti_synergies\":[],\"card_types\":[\"Action\"],\"art_url\":[\"http://wiki.dominionstrategy.com/images/archive/2/29/20161006150258%21Chapel.jpg\",\"http://wiki.dominionstrategy.com/images/2/29/Chapel.jpg\"]}");
    }

    #[test]
    fn test_card_json_from_name_lowercase() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!(super::card_json_from_name("chapel".to_string()))).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), 
            "{\"id\":101,\"name\":\"Chapel\",\"supply_card\":true,\"basic_card\":false,\"card_counts\":[10,10,10,10,10],\"set_id\":1,\"editions\":[1,2],\"card_tags\":[\"IsAction\",\"Costs2\",\"NetLoseCards\",\"TrashFromHand\",\"TrashMultipleFromHand\",\"TrashTreasre\",\"TrashAction\",\"TrashVictory\",\"TrashCurse\"],\"kingdom_requirements\":[],\"kingdom_synergies\":[],\"kingdom_anti_synergies\":[],\"card_types\":[\"Action\"],\"art_url\":[\"http://wiki.dominionstrategy.com/images/archive/2/29/20161006150258%21Chapel.jpg\",\"http://wiki.dominionstrategy.com/images/2/29/Chapel.jpg\"]}");
    }

    #[test]
    fn test_card_json_from_name_unknowen_name() {
        let client = Client::tracked(launch_app()).expect("valid rocket instance");
        let response = client.get(uri!(super::card_json_from_name("What are you doing? Why on earth is this your card name? Now I need to fix my test!".to_string()))).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Unknown card name".to_string());
    }
}