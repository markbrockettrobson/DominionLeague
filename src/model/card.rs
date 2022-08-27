use rocket::serde::{Deserialize, Serialize};

use super::{card_type::CardType, card_tag::CardTag};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    pub id: u8,
    pub name: String,
    pub set_id: u8,
    pub editions: Vec<u8>,
    pub card_tags: Vec<CardTag>,
    pub kindom_requirements: Vec<Vec<CardTag>>,
    pub kindom_synergies: Vec<Vec<CardTag>>,
    pub kindom_anti_synergies: Vec<Vec<CardTag>>,
    pub card_types: Vec<CardType>
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
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = card_one.clone();
        card_one.id = 20;
        assert_ne!(card_one.id, card_two.id);
        assert_eq!(card_one.name, card_two.name);
        assert_eq!(card_one.kindom_anti_synergies, card_two.kindom_anti_synergies);
    }
    
    #[test]
    fn test_clone_from() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let mut card_two = Card { 
            id: 2,
            name: "test name 2".to_string(),
            set_id: 2,
            editions: [2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

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
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one == card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_id() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 2,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_name() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "not test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_set_id() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 1,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_editions() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_card_tags() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::WillGainBuy].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_kindom_requirements() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck, CardTag::CurseAttack].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_kindom_synergies() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_kindom_anti_synergies() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_card_types() {
        let card_one = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let card_two = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Reaction, CardType::Action ,CardType::Victory].to_vec()

        };
        assert!(card_one != card_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()

        };
        let json = serde_json::to_string(&card).unwrap();
        
        assert_eq!(
            json,
            "{\"id\":1,\"name\":\"test name\",\"set_id\":0,\"editions\":[1,2],\"card_tags\":[\"CostsAction\"],\"kindom_requirements\":[[\"AddCardToTopOfDeck\"]],\"kindom_synergies\":[[\"CanReplaceAction\"]],\"kindom_anti_synergies\":[[\"WillReplaceAction\"]],\"card_types\":[\"Action\",\"Attack\",\"Curse\"]}");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()
        };
        let json = "{\"id\":1,\"name\":\"test name\",\"set_id\":0,\"editions\":[1,2],\"card_tags\":[\"CostsAction\"],\"kindom_requirements\":[[\"AddCardToTopOfDeck\"]],\"kindom_synergies\":[[\"CanReplaceAction\"]],\"kindom_anti_synergies\":[[\"WillReplaceAction\"]],\"card_types\":[\"Action\",\"Attack\",\"Curse\"]}";
        let json_set: Card = serde_json::from_str(json).unwrap();

        assert_eq!(json_set, card);
    }

    #[test]
    fn test_fmt() {
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            set_id: 0,
            editions: [1, 2].to_vec(),
            card_tags: [CardTag::CostsAction].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kindom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kindom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec()
        };
        assert_eq!(
            format!("{card:?}"), 
            "Card { id: 1, name: \"test name\", set_id: 0, editions: [1, 2], card_tags: [CostsAction], kindom_requirements: [[AddCardToTopOfDeck]], kindom_synergies: [[CanReplaceAction]], kindom_anti_synergies: [[WillReplaceAction]], card_types: [Action, Attack, Curse] }");
    }

}
