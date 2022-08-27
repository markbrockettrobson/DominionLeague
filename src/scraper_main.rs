pub mod model;

use self::model::data_loaders::set_data_loader::get_set_vec;
use self::model::scrapers::scrape_set_files::scrape_set_files;

#[mutants::skip]
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    println!("Reading Sets...");
    let sets = get_set_vec();
    println!("Loaded {} sets", sets.len());
    let mut count: u8 = 0; 
    for set in sets.clone(){
        
        println!("Scraping {}", set.name);
        match scrape_set_files(&set).await {
            Ok(_) => count+= 1,
            Err(error) => println!("Had Error {:?}!", error),
        }
    }
    println!("Compleated {}/{} sets without problems", count ,sets.len());
}
