use std::include_bytes;
use rocket::serde::json::serde_json;

use super::super::card::Card;


pub static BASE_CARD_JSON_BYTES: &[u8] = include_bytes!("../raw_data/basic_cards.json");

#[allow(dead_code)]
pub fn get_card_vec() -> Vec<Card> {
    let set_json = String::from_utf8_lossy(BASE_CARD_JSON_BYTES).into_owned();
    serde_json::from_str(&set_json).unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_cards_vec_loads_at_least_one_set() {
        let cards = get_card_vec();
        assert_eq!(cards[0].id, 0);
    }

    #[test]
    fn test_no_missing_cards_id() {
        let cards = get_card_vec();
        for (index, card) in cards.iter().enumerate() {
            assert_eq!(card.id, index as u8);
        }
    }
}
