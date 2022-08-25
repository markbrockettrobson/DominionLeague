use std::{fs::{File, create_dir_all}, io::{self, Cursor}, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use reqwest;

#[allow(dead_code)]
pub(crate) async fn download_file_to_path(
    file_path_str: &str, 
    image_url: &str) -> Result<()> {
        let request_bytes = reqwest::get(image_url).await?.bytes().await?;
        save_byes_to_file(file_path_str, request_bytes)?;
        Ok(())
    }

fn save_byes_to_file(file_path_str: &str, mut bytes: rocket::http::hyper::body::Bytes) -> Result<()> {
    let file_path = Path::new(file_path_str);
    match file_path.parent(){
        Some(parent) => {
            create_dir_all(parent)?;
        },
        None => (),
    }
    let mut file_pointer = File::create(file_path)?;
    let mut cursor = Cursor::new(&mut bytes);
    io::copy(&mut cursor, &mut file_pointer)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use tokio;
    use file_diff::{diff};
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn test_download_file_to_path() {
        let logo_url = "https://github.githubassets.com/favicons/favicon.svg";
        let expected_sample_file_path = "src/model/scrapers/sample_files/github.svg";
        let file_path = "src/model/scrapers/sample_files/downloaded.svg";

        match fs::remove_file(file_path) {
            Ok(_) => (),
            Err(_) => (),
        }
        match download_file_to_path(file_path, logo_url).await {
            Ok(_) => (),
            Err(_) => {
                panic!("Did not work.")
            }
        }
        
        assert!(diff(expected_sample_file_path, file_path));
        match fs::remove_file(file_path) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}
