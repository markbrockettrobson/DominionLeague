use std::include_bytes;
use rocket::serde::json::serde_json;

use super::set::Set;


pub static SET_JSON_BYTES: &[u8] = include_bytes!("raw_data/sets.json");

#[allow(dead_code)]
fn get_set_vec() -> Vec<Set> {
    let set_json = String::from_utf8_lossy(SET_JSON_BYTES).into_owned();
    serde_json::from_str(&set_json).unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_set_vec_loads_at_least_one_set() {
        let sets = get_set_vec();
        assert_eq!(sets[0].id, 0);
        assert_eq!(sets[0].name, "Dominion");
        assert_eq!(sets[0].editions, 2);
    }

    #[test]
    fn test_no_missing_set_id() {
        let sets = get_set_vec();
        for (index, set) in sets.iter().enumerate() {
            assert_eq!(set.id, index as u8);
        }
    }
}
