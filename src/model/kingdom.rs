use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Kingdom {
    pub supply_card_ids: Vec<u16>,
    pub basic_card_ids: Vec<u16>,
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::serde::json::serde_json;

    #[test]
    fn test_clone() {
        let mut kingdom_one = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let kingdom_two = kingdom_one.clone();
        
        kingdom_one.supply_card_ids = [10, 12].to_vec();
        assert_eq!(kingdom_one.basic_card_ids, kingdom_two.basic_card_ids);
        assert_ne!(kingdom_one.supply_card_ids, kingdom_two.supply_card_ids);

    }
    
    #[test]
    fn test_clone_from() {
        let kingdom_one = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let mut kingdom_two = Kingdom { 
            supply_card_ids: [13].to_vec(),
            basic_card_ids: [0, 1, 2, 10].to_vec()
        };
        kingdom_two.clone_from(&kingdom_one);
        assert_eq!(kingdom_two, kingdom_one);
    }

    #[test]
    fn test_eq_true() {
        let kingdom_one = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let kingdom_two = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        assert!(kingdom_two == kingdom_one);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_supply_cards() {
        let kingdom_one = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let kingdom_two = Kingdom { 
            supply_card_ids: [10, 11, 13, 18].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        assert!(kingdom_two != kingdom_one);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_basic_cards() {
        let kingdom_one = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let kingdom_two = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 2].to_vec()
        };
        assert!(kingdom_two != kingdom_one);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let kingdom = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let json = serde_json::to_string(&kingdom).unwrap();
        
        assert_eq!(
            json,
            "{\"supply_card_ids\":[10,11,13],\"basic_card_ids\":[0,1,2]}");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let kingdom = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        let json = "{\"supply_card_ids\":[10,11,13],\"basic_card_ids\":[0,1,2]}";
        let json_set: Kingdom = serde_json::from_str(json).unwrap();
        assert_eq!(json_set, kingdom);
    }

    #[test]
    fn test_fmt() {
        
        let kingdom = Kingdom { 
            supply_card_ids: [10, 11, 13].to_vec(),
            basic_card_ids: [0, 1, 2].to_vec()
        };
        assert_eq!(
            format!("{kingdom:?}"), 
            "Kingdom { supply_card_ids: [10, 11, 13], basic_card_ids: [0, 1, 2] }");
    }


}
