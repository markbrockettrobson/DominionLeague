pub mod model;

use self::model::data_loaders::set_data_loader::get_set_vec;
use self::model::data_loaders::card_data_loader::get_card_vec;
use self::model::scrapers::scrape_set_files::scrape_set_files;
use self::model::scrapers::scrape_card_files::scrape_card_files;


#[mutants::skip]
#[allow(dead_code)]
async fn get_all_sets(){
    println!("Reading Sets...");
    let sets = get_set_vec();
    println!("Loaded {} sets", sets.len());
    let mut set_count: u8 = 0; 
    for set in sets.clone(){
        
        println!("Scraping {}", set.name);
        match scrape_set_files(&set).await {
            Ok(_) => set_count+= 1,
            Err(error) => println!("Had Error {:?}!", error),
        }
    }
    println!("Compleated {}/{} sets without problems", set_count ,sets.len());
}

#[mutants::skip]
#[allow(dead_code)]
async fn get_all_cards(){
    println!("Reading cards...");
    let cards = get_card_vec();
    println!("Loaded {} cards", cards.len());
    let mut card_count: u8 = 0; 
    for card in cards.clone(){
        
        println!("Scraping {}", card.name);
        match scrape_card_files(&card).await {
            Ok(_) => card_count+= 1,
            Err(error) => println!("Had Error {:?}!", error),
        }
    }
    println!("Compleated {}/{} cards without problems", card_count ,cards.len());
}


#[mutants::skip]
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    get_all_cards().await;
}
