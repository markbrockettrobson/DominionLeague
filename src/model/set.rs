use rocket::serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Set {
    pub id: u8,
    pub name: String,
    pub editions: Vec<u8>,
    pub cover_art_url: Vec<String>,
    pub rule_book_url: Vec<String>,
    pub icon_url: Vec<String>,
}

impl Set {
    pub fn new(
        id: u8,
        name: String,
        editions: Vec<u8>,
        cover_art_url: Vec<String>,
        rule_book_url: Vec<String>,
        icon_url: Vec<String>
    ) -> Self {
        if 
            editions.is_empty() ||
            editions.len() != cover_art_url.len() ||
            editions.len() != rule_book_url.len() ||
            editions.len() != icon_url.len()
        {
            panic!("Must have one or more editions each with cover_art_url, rule_book_url and icon_url.")
        } 

        Self {
            id,
            name,
            editions,
            cover_art_url,
            rule_book_url,
            icon_url
        }
    }
}

pub trait SetFilePaths {
    fn get_cover_art_path(&self, edition: u8) -> String;
    fn get_rule_book_path(&self, edition: u8) -> String;
    fn get_icon_path(&self, edition: u8) -> String;
}

impl SetFilePaths for Set {
    fn get_cover_art_path(&self, edition: u8) -> String {
        if !self.editions.contains(&edition){ panic!("Unkowen edition."); }
        let unsafe_filename = format!("../scraped_data/{0}_{1}/{0}_{1}_cover_art.png", self.name, edition);
        let regex = Regex::new(r"[\s<>:;',?*|\\]").unwrap();
        regex.replace_all(&unsafe_filename, "-").as_ref().to_string()
    }
    fn get_rule_book_path(&self, edition: u8) -> String{
        if !self.editions.contains(&edition){ panic!("Unkowen edition."); }
        let unsafe_filename = format!("../scraped_data/{0}_{1}/{0}_{1}_rules.pdf", self.name, edition);
        let regex = Regex::new(r"[\s<>:;',?*|\\]").unwrap();
        regex.replace_all(&unsafe_filename, "-").as_ref().to_string()   
    }
    fn get_icon_path(&self, edition: u8) -> String{
        if !self.editions.contains(&edition){ panic!("Unkowen edition."); }
        let unsafe_filename = format!("../scraped_data/{0}_{1}/{0}_{1}_icon.png", self.name, edition);
        let regex = Regex::new(r"[\s<>:;',?*|\\]").unwrap();
        regex.replace_all(&unsafe_filename, "-").as_ref().to_string()    
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::serde::json::serde_json;

    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_empty_editions() {
        let _ = Set::new( 
            1, 
            "test name".to_string(), 
            vec![], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
    }

    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_empty_cover_art_url() {
        let _ = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec![], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
    }

    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_empty_rule_book_url() {
        let _ = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec![], 
            vec!["www.icon_url.com".to_owned()]
        );
    }
    
    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_empty_rule_icon_url() {
        let _ = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec![]
        );
    }
    
    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_different_length_cover_art_url() {
        let _ = Set::new(
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
    }    

    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_different_length_rule_book_url() {
        let _ = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
    }

    #[test]
    #[should_panic = "Must have one or more editions each with cover_art_url, rule_book_url and icon_url."]
    fn test_panic_on_different_length_icon_url() {
        let _ = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
    }

    #[test]
    fn test_clone() {
        let mut set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        let set_two = set_one.clone();
        set_one.id = 20;
        assert_ne!(set_one.id, set_two.id);
        assert_eq!(set_one.name, set_two.name);
        assert_eq!(set_one.editions, set_two.editions);
        assert_eq!(set_one.cover_art_url, set_two.cover_art_url);
        assert_eq!(set_one.rule_book_url, set_two.rule_book_url);
        assert_eq!(set_one.icon_url, set_two.icon_url);
    }
    
    #[test]
    fn test_clone_from() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        let mut set_two = Set::new(
            1,
            "set not two".to_string(),
            vec![4],
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        set_two.clone_from(&set_one);
        assert_eq!(set_two.id, 1);
        assert_eq!(set_two.name, "test name");
        assert_eq!(set_two.editions, vec![1]);
    }

    #[test]
    fn test_eq_true() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        let set_two = Set::new(  
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        assert!(set_one == set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_id() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        let set_two = Set::new(  
            2, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_name() {
        let set_one = Set::new(
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()],
            vec!["www.icon_url.com".to_owned()]
        );
        let set_two = Set::new( 
            1, 
            "test name 2".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url.com".to_owned()]
        );
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_edition() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 3], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()],  
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        let set_two = Set::new(
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()],  
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_cover_art_url() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        let set_two = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.1cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_rule_book_url() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        let set_two = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.1rule_book_url2.com".to_owned()], 
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_icon_url() {
        let set_one = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url.com".to_owned(), "www.icon_url2.com".to_owned()]
        );
        let set_two = Set::new(  
            1, 
            "test name".to_string(), 
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.1rule_book_url2.com".to_owned()],
            vec!["www.icon_url2.com".to_owned(), "www.icon_url.com".to_owned()]
        );
        assert!(set_one != set_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_serialize() {
        let set = Set::new( 
            1, 
            "test name".to_string(), 
            vec![1], 
            vec!["www.cover_art_url.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned()]
        );
        let json = serde_json::to_string(&set).unwrap();
        
        assert_eq!(
            json,
            "{\"id\":1,\"name\":\"test name\",\"editions\":[1],\"cover_art_url\":[\"www.cover_art_url.com\"],\"rule_book_url\":[\"www.rule_book_url.com\"],\"icon_url\":[\"www.icon_url2.com\"]}"
        );
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_deserialize() {
        let set = Set::new( 
            1, 
            "test name".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        let json = "{\"id\":1,\"name\":\"test name\",\"editions\":[1, 2],\"cover_art_url\":[\"www.cover_art_url.com\", \"www.cover_art_url2.com\"],\"rule_book_url\":[\"www.rule_book_url.com\", \"www.rule_book_url2.com\"],\"icon_url\":[\"www.icon_url2.com\", \"www.icon_url3.com\"]}";
        let json_set: Set = serde_json::from_str(json).unwrap();

        assert_eq!(json_set, set);
    }

    #[test]
    fn test_fmt() {
        let set = Set::new( 
            1, 
            "test name".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url6.com".to_owned()]
        );
        assert_eq!(
            format!("{set:?}"), 
            "Set { id: 1, name: \"test name\", editions: [1, 2], cover_art_url: [\"www.cover_art_url.com\", \"www.cover_art_url2.com\"], rule_book_url: [\"www.rule_book_url.com\", \"www.rule_book_url2.com\"], icon_url: [\"www.icon_url2.com\", \"www.icon_url6.com\"] }"
        );
    }

    #[test]
    fn test_get_cover_art_path() {
        let set = Set::new( 
            1, 
            "test name'one".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        assert_eq!(set.get_cover_art_path(1), "../scraped_data/test-name-one_1/test-name-one_1_cover_art.png");
        assert_eq!(set.get_cover_art_path(2), "../scraped_data/test-name-one_2/test-name-one_2_cover_art.png");
    }
        
    #[test]
    #[should_panic = "Unkowen edition."]
    fn test_get_cover_art_path_unknowen_edition() {
        let set = Set::new( 
            1, 
            "test name'one".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        let _ = set.get_cover_art_path(45);
    }

    #[test]
    fn test_get_rule_book_path() {
        let set = Set::new( 
            1, 
            "test?name|one".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        assert_eq!(set.get_rule_book_path(1), "../scraped_data/test-name-one_1/test-name-one_1_rules.pdf");
        assert_eq!(set.get_rule_book_path(2), "../scraped_data/test-name-one_2/test-name-one_2_rules.pdf");
    }
        
    #[test]
    #[should_panic = "Unkowen edition."]
    fn test_get_rule_book_path_unknowen_edition() {
        let set = Set::new( 
            1, 
            "test name'one".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        let _ = set.get_rule_book_path(45);
    }

    #[test]
    fn test_get_icon_path() {
        let set = Set::new( 
            1, 
            "test name'one".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        assert_eq!(set.get_icon_path(1), "../scraped_data/test-name-one_1/test-name-one_1_icon.png");
        assert_eq!(set.get_icon_path(2), "../scraped_data/test-name-one_2/test-name-one_2_icon.png");
    }
        
    #[test]
    #[should_panic = "Unkowen edition."]
    fn test_get_icon_path_unknowen_edition() {
        let set = Set::new( 
            1, 
            "test name'one".to_string(),
            vec![1, 2], 
            vec!["www.cover_art_url.com".to_owned(), "www.cover_art_url2.com".to_owned()], 
            vec!["www.rule_book_url.com".to_owned(), "www.rule_book_url2.com".to_owned()], 
            vec!["www.icon_url2.com".to_owned(), "www.icon_url3.com".to_owned()]
        );
        let _ = set.get_icon_path(45);
    }

}
