use std::collections::HashMap;

use crate::model::{card::Card, data_loaders::card_data_loader::{get_all_card_id_map, get_all_card_name_map}};

pub struct CardData {
    pub id_card_map: HashMap<u16, Card>,
    pub name_card_map: HashMap<String, Card>
}

pub fn build_card_data() -> CardData {
    CardData { 
        id_card_map: get_all_card_id_map(),
        name_card_map: get_all_card_name_map()
    }
}

#[cfg(test)]
mod test {
    use crate::model::data_loaders::card_data_loader::get_all_card_vec;

    use super::build_card_data;

    #[test]
    fn test_card_id_map() {
        let card_data = build_card_data();
        for card in get_all_card_vec().iter() {
            assert_eq!(card, &card_data.id_card_map[&card.id]);
        }
    }

    
    #[test]
    fn test_card_name_map() {
        let card_data = build_card_data();
        for card in get_all_card_vec().iter() {
            assert_eq!(card, &card_data.name_card_map[&card.name]);
            assert_eq!(card, &card_data.name_card_map[&card.name.to_ascii_lowercase()]);
        }
    }
}
