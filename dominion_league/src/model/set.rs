use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Set {
    pub id: u8,
    pub name: String,
    pub editions: u8,
}


#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use rocket::serde::json::serde_json;

    #[test]
    fn test_clone() {
        let mut set_one = Set { id: 1, name: "test name".to_string(), editions: 1 };
        let set_two = set_one.clone();
        set_one.id = 20;
        assert_ne!(set_one.id, set_two.id);
        assert_eq!(set_one.name, set_two.name);
        assert_eq!(set_one.editions, set_two.editions);
    }
    
    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_clone_from() {
        let set_one = Set { id: 2, name: "set two".to_string(), editions: 2 };
        let set_two = set_one.clone();
        assert_eq!(set_two.id, 2);
        assert_eq!(set_two.name, "set two");
        assert_eq!(set_two.editions, 2);
    }

    #[test]
    fn test_eq_true() {
        let set_one = Set { id: 2, name: "set".to_string(), editions: 2 };
        let set_two = Set { id: 2, name: "set".to_string(), editions: 2 };
        assert!(set_one == set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_id() {
        let set_one = Set { id: 1, name: "set".to_string(), editions: 2 };
        let set_two = Set { id: 2, name: "set".to_string(), editions: 2 };
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_name() {
        let set_one = Set { id: 2, name: "set 1".to_string(), editions: 2 };
        let set_two = Set { id: 2, name: "set 2".to_string(), editions: 2 };
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_edition() {
        let set_one = Set { id: 2, name: "set".to_string(), editions: 1 };
        let set_two = Set { id: 2, name: "set".to_string(), editions: 2 };
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let set = Set { id: 2, name: "set".to_string(), editions: 1 };
        let json = serde_json::to_string(&set).unwrap();
        
        assert_eq!(json,  "{\"id\":2,\"name\":\"set\",\"editions\":1}");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let set = Set { id: 2, name: "set".to_string(), editions: 1 };
        let json = "{\"id\":2,\"name\":\"set\",\"editions\":1}";
        let json_set: Set = serde_json::from_str(json).unwrap();

        assert_eq!(json_set, set);
    }

    proptest! {
        #[test]
        fn test_fmt(test_id: u8, test_name: String, test_edition: u8) {
            let set = Set { id: test_id, name: test_name.to_string(), editions: test_edition };
            assert_eq!(
                format!("{set:?}"), 
                format!("Set {{ id: {}, name: {:?}, editions: {} }}", test_id, test_name, test_edition));
        }
    }
}
