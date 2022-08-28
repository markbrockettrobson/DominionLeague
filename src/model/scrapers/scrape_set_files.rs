use crate::model::{set::{Set, SetFilePaths}};

use super::scraper_helpers::download_file_to_path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[allow(dead_code)]
pub(crate) async fn scrape_set_files(set: &Set) -> Result<()> {
    for (index, edition) in set.editions.iter().enumerate() {
        download_file_to_path(
            &set.get_cover_art_path(*edition),
            set.cover_art_url.get(index).unwrap()).await?;
        download_file_to_path(
            &set.get_icon_path(*edition),
            set.icon_url.get(index).unwrap()).await?;
        download_file_to_path(
            &set.get_rule_book_path(*edition),
            set.rule_book_url.get(index).unwrap()).await?;
    }
    Ok(())
} 


#[cfg(test)]
mod test {
    use std::fs::remove_dir_all;

    use tokio;
    use file_diff::diff;
    use crate::model::set::{Set, SetFilePaths};
    use super::scrape_set_files;

    #[tokio::test]
    async fn test_scrape_set_files() {
        let _ = remove_dir_all("src\\model\\scraped_data\\test-name-one_13");
        let _ = remove_dir_all("src\\model\\scraped_data\\test-name-one_21");
        let logo_url = "https://github.githubassets.com/favicons/favicon.svg";
        let set = Set::new( 
            1, 
            "test name'one".to_string(),
            vec![13, 21], 
            vec![logo_url.to_string(), logo_url.to_string()], 
            vec![logo_url.to_string(), logo_url.to_string()],  
            vec![logo_url.to_string(), logo_url.to_string()], 
        );

        let files = [
            set.get_cover_art_path(13),
            set.get_rule_book_path(13),
            set.get_icon_path(13),
            set.get_cover_art_path(21),
            set.get_rule_book_path(21),
            set.get_icon_path(21),
        ];
        let _ = scrape_set_files(&set).await;

        let expected_sample_file_path = "src/model/scrapers/sample_files/github.svg";


        for file in files {
            println!("{}", file);
            assert!(diff(expected_sample_file_path, &file));
        }
        let _ = remove_dir_all("src\\model\\scraped_data\\test-name-one_13");
        let _ = remove_dir_all("src\\model\\scraped_data\\test-name-one_21");

    }

}
