use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum CardType{
    Action,
    Treasure,
    Victory,
    Curse,
    Attack,
    Reaction
}


#[cfg(test)]
mod test {
    use super::*;
    use rocket::serde::json::serde_json;

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut card_type_one = CardType::Action;
        let card_type_two = card_type_one.clone();
        card_type_one = CardType::Attack;
        assert_ne!(card_type_one, card_type_two);
        assert_eq!(card_type_two,CardType::Action);
    }
    
    #[test]
    fn test_clone_from() {
        let card_type_one = CardType::Action;
        let mut card_type_two = CardType::Curse;
        card_type_two.clone_from(&card_type_one);
        assert_eq!(card_type_two, CardType::Action);
    }

    #[test]
    fn test_eq_true() {
        let card_type_one = CardType::Action;
        let card_type_two = CardType::Action;
        assert!(card_type_one == card_type_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false() {
        let card_type_one = CardType::Action;
        let card_type_two = CardType::Attack;
        assert!(card_type_one != card_type_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let card_type = CardType::Action;
        let json = serde_json::to_string(&card_type).unwrap();
        
        assert_eq!(json,  "\"Action\"");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let card_type = CardType::Action;
        let json = "\"Action\"";
        let json_card_type: CardType = serde_json::from_str(json).unwrap();

        assert_eq!(json_card_type, card_type);
    }

    #[test]
    fn test_fmt() {
        let card_type = CardType::Action;
        assert_eq!(
            format!("{card_type:?}"), 
            "Action");
    }
}
