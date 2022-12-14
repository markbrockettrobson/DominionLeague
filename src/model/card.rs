use std::{fs::canonicalize, path::PathBuf};

use regex::Regex;
use rocket::serde::{Deserialize, Serialize};

use super::{card_type::CardType, card_tag::CardTag};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    pub id: u16,
    pub name: String,
    pub supply_card: bool,
    pub basic_card: bool,
    pub card_counts: [u8; 5],// for games with players 2,3,4,5,6  
    pub set_id: u8,
    pub editions: Vec<u8>,
    pub card_tags: Vec<CardTag>,
    pub kingdom_requirements: Vec<Vec<CardTag>>,
    pub kingdom_synergies: Vec<Vec<CardTag>>,
    pub kingdom_anti_synergies: Vec<Vec<CardTag>>,
    pub card_types: Vec<CardType>,
    pub art_url: Vec<String>
}

pub trait CardFilepath {
    fn get_art_path(&self, edition: u8) -> String;
    fn get_art_file_name(&self, edition: u8) -> String;
}

impl CardFilepath for Card {
    fn get_art_path(&self, edition: u8) -> String {
        let safe_card_name = self.get_art_file_name(edition);
        let target_folder = canonicalize("src/model/scraped_data/").unwrap();
        let mut path = PathBuf::new();
        path.push(target_folder);
        path.push("cards");
        path.push(safe_card_name);
        path.set_extension("jpeg");
        path.as_path().to_string_lossy().into_owned()
    }
    
    fn get_art_file_name(&self, edition: u8) -> String {
        if !self.editions.contains(&edition){ panic!("Unkowen edition."); }
        let regex = Regex::new(r"[\s<>:;',?*|\\]").unwrap();
        regex.replace_all(
            format!("{0}_{1}", self.name, edition).as_str(), 
            "-"
        ).as_ref().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::serde::json::serde_json;

    #[test]
    fn test_clone() {
        let mut card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],  
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = card_one.clone();
        card_one.id = 20;
        assert_ne!(card_one.id, card_two.id);
        assert_eq!(card_one.name, card_two.name);
        assert_eq!(card_one.basic_card, card_two.basic_card);
        assert_eq!(card_one.kingdom_anti_synergies, card_two.kingdom_anti_synergies);
        assert_eq!(card_one.art_url, card_two.art_url);
    }
    
    #[test]
    fn test_clone_from() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],  
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let mut card_two = Card { 
            id: 2,
            name: "test name 2".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 2,
            editions: [2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        card_two.clone_from(&card_one);
        assert_eq!(card_two.id, 1);
        assert_eq!(card_two.name, "test name");
    }

    #[test]
    fn test_eq_true() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one == card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_id() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 2,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_name() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "not test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_supply_card() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: false,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_basic_card() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: false,
            basic_card: true,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: false,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_card_counts() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 1,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 12, 12],
            set_id: 1,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_set_id() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 1,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_editions() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_card_tags() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::WillGainBuy].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_kingdom_requirements() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck, CardTag::CurseAttack].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_kindom_synergies() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_kindom_anti_synergies() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_card_types() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Reaction, CardType::Action ,CardType::Victory].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_art_url() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image4.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let json = serde_json::to_string(&card).unwrap();
        
        assert_eq!(
            json,
            "{\"id\":1,\"name\":\"test name\",\"supply_card\":true,\"basic_card\":false,\"card_counts\":[10,10,10,10,10],\"set_id\":0,\"editions\":[1,2],\"card_tags\":[\"Costs4\"],\"kingdom_requirements\":[[\"AddCardToTopOfDeck\"]],\"kingdom_synergies\":[[\"CanReplaceAction\"]],\"kingdom_anti_synergies\":[[\"WillReplaceAction\"]],\"card_types\":[\"Action\",\"Attack\",\"Curse\"],\"art_url\":[\"www.image1.com\",\"www.image2.com\"]}");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let json = "{\"id\":1,\"name\":\"test name\",\"supply_card\":true,\"basic_card\":false,\"card_counts\":[10,10,10,10,10],\"set_id\":0,\"editions\":[1,2],\"card_tags\":[\"Costs4\"],\"kingdom_requirements\":[[\"AddCardToTopOfDeck\"]],\"kingdom_synergies\":[[\"CanReplaceAction\"]],\"kingdom_anti_synergies\":[[\"WillReplaceAction\"]],\"card_types\":[\"Action\",\"Attack\",\"Curse\"],\"art_url\":[\"www.image1.com\",\"www.image2.com\"]}";
        let json_set: Card = serde_json::from_str(json).unwrap();

        assert_eq!(json_set, card);
    }

    #[test]
    fn test_fmt() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert_eq!(
            format!("{card:?}"), 
            "Card { id: 1, name: \"test name\", supply_card: true, basic_card: false, card_counts: [10, 10, 10, 10, 10], set_id: 0, editions: [1, 2], card_tags: [Costs4], kingdom_requirements: [[AddCardToTopOfDeck]], kingdom_synergies: [[CanReplaceAction]], kingdom_anti_synergies: [[WillReplaceAction]], card_types: [Action, Attack, Curse], art_url: [\"www.image1.com\", \"www.image2.com\"] }");
    }

    #[test]
    fn test_get_art_path() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert!(
            card.get_art_path(1).ends_with("\\cards\\test-name_1.jpeg") ||
            card.get_art_path(1).ends_with("/cards/test-name_1.jpeg") 
        );assert!(
            card.get_art_path(2).ends_with("\\cards\\test-name_2.jpeg") ||
            card.get_art_path(2).ends_with("/cards/test-name_2.jpeg") 
        );
    }
        
    #[test]
    #[should_panic = "Unkowen edition."]
    fn test_get_art_path_unknowen_edition() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let _ = card.get_art_path(45);
    }

    
    #[test]
    fn test_get_art_file_name() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        assert_eq!(card.get_art_file_name(1), "test-name_1");
        assert_eq!(card.get_art_file_name(2), "test-name_2");
    }
        
    #[test]
    #[should_panic = "Unkowen edition."]
    fn test_get_art_file_name_unknowen_edition() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kingdom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: ["www.image1.com".to_string(), "www.image2.com".to_string()].to_vec()
        };
        let _ = card.get_art_file_name(45);
    }
}
