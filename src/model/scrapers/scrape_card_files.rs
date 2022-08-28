use crate::model::{card::{Card, CardFilepath}};

use super::scraper_helpers::download_file_to_path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[allow(dead_code)]
pub(crate) async fn scrape_card_files(card: &Card) -> Result<()> {
    for (index, edition) in card.editions.iter().enumerate() {
        download_file_to_path(
            &card.get_art_path(*edition),
            card.art_url.get(index).unwrap()).await?;
    }
    Ok(())
} 


#[cfg(test)]
mod test {
    use std::fs::remove_file;

    use tokio;
    use file_diff::diff;
    use crate::model::{card_tag::CardTag, card_type::CardType, card::{Card, CardFilepath}};
    use super::scrape_card_files;

    #[tokio::test]
    async fn test_scrape_card_files() {
        let _ = remove_file("src\\model\\scraped_data\\cards\\test-name_13.jpeg");
        let _ = remove_file("src\\model\\scraped_data\\cards\\test-name_21.jpeg");
        let logo_url = "https://github.githubassets.com/favicons/favicon.svg";
        let card = Card { 
            id: 1,
            name: "test name".to_string(),
            supply_card: true,
            basic_card: false,
            card_counts: [10, 10, 10, 10, 10],  
            set_id: 0,
            editions: [13, 21].to_vec(),
            card_tags: [CardTag::Costs4].to_vec(),
            kindom_requirements: [[CardTag::AddCardToTopOfDeck].to_vec()].to_vec(),
            kingdom_synergies: [[CardTag::CanReplaceAction].to_vec()].to_vec(),
            kingdom_anti_synergies: [[CardTag::WillReplaceAction].to_vec()].to_vec(),
            card_types: [CardType::Action, CardType::Attack ,CardType::Curse].to_vec(),
            art_url: [logo_url.to_string(), logo_url.to_string()].to_vec()
        };
        let files = [
            card.get_art_path(13),
            card.get_art_path(21),
        ];
        let _ = scrape_card_files(&card).await;

        let expected_sample_file_path = "src/model/scrapers/sample_files/github.svg";

        for file in files {
            println!("{}", file);
            assert!(diff(expected_sample_file_path, &file));
        }
        let _ = remove_file("src\\model\\scraped_data\\cards\\test-name_13.jpeg");
        let _ = remove_file("src\\model\\scraped_data\\cards\\test-name_21.jpeg");

    }

}
