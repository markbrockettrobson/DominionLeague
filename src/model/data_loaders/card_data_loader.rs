use std::include_bytes;
use rocket::serde::json::serde_json;

use super::super::card::Card;


pub static BASE_CARD_JSON_BYTES: &[u8] = include_bytes!("../raw_data/basic_cards.json");
pub static DOMINION_CARD_JSON_BYTES: &[u8] = include_bytes!("../raw_data/dominion_cards.json");


#[allow(dead_code)]
pub fn get_base_card_vec() -> Vec<Card> {
    let set_json = String::from_utf8_lossy(BASE_CARD_JSON_BYTES).into_owned();
    serde_json::from_str(&set_json).unwrap()
}

#[allow(dead_code)]
pub fn get_dominion_card_vec() -> Vec<Card> {
    let set_json = String::from_utf8_lossy(DOMINION_CARD_JSON_BYTES).into_owned();
    serde_json::from_str(&set_json).unwrap()
}

#[allow(dead_code)]
pub fn get_all_card_vec() -> Vec<Card> {
    let mut all_cards = vec![];
    all_cards.extend(get_base_card_vec());
    all_cards.extend(get_dominion_card_vec());
    all_cards
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_all_card_vec() {
        let all_cards = get_all_card_vec();
        let base_cards = get_base_card_vec();
        let dominion_cards = get_dominion_card_vec();

        assert_eq!(all_cards.len(), base_cards.len() + dominion_cards.len());
    }

    #[test]
    fn test_get_base_card_vec_loads_at_least_one_card() {
        let cards = get_base_card_vec();
        assert_eq!(cards[0].id, 0);
    }

    #[test]
    fn test_get_base_card_vec_no_missing_cards_id() {
        let cards = get_base_card_vec();
        for (index, card) in cards.iter().enumerate() {
            assert_eq!(card.id, index as u16);
        }
    }
    
    #[test]
    fn test_get_dominion_card_vec_loads_at_least_one_card() {
        let cards = get_dominion_card_vec();
        assert_eq!(cards[0].id, 100);
    }

    #[test]
    fn test_get_dominion_card_vec_no_missing_cards_id() {
        let cards = get_dominion_card_vec();
        for (index, card) in cards.iter().enumerate() {
            assert_eq!(card.id, (index + 100) as u16);
        }
    }
}
