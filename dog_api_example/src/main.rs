use serde::Deserialize;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum FetchError {
    NetworkError(String),
    ApiError(String),
    JsonError(String),
    DownloadError(String),
    IoError(String),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::NetworkError(e) => write!(f, "Network error: {}", e),
            FetchError::ApiError(e) => write!(f, "API error: {}", e),
            FetchError::JsonError(e) => write!(f, "JSON parse error: {}", e),
            FetchError::DownloadError(e) => write!(f, "Download error: {}", e),
            FetchError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for FetchError {}

fn fetch_random_dog_image() -> Result<DogImage, FetchError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    let response = ureq::get(url)
        .timeout(Duration::from_secs(10))
        .call()
        .map_err(|e| FetchError::NetworkError(e.to_string()))?;

    if response.status() != 200 {
        return Err(FetchError::ApiError(format!(
            "Unexpected HTTP status: {}",
            response.status()
        )));
    }

    response
        .into_json::<DogImage>()
        .map_err(|e| FetchError::JsonError(e.to_string()))
}

fn download_image(url: &str, filename: &str) -> Result<(), FetchError> {
    let response = ureq::get(url)
        .timeout(Duration::from_secs(20))
        .call()
        .map_err(|e| FetchError::DownloadError(e.to_string()))?;

    let mut reader = response.into_reader();
    let mut file = File::create(filename).map_err(|e| FetchError::IoError(e.to_string()))?;
    let _bytes_copied = copy(&mut reader, &mut file).map_err(|e| FetchError::IoError(e.to_string()))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🐕 Random Dog Image Downloader");
    println!("==============================\n");

    create_dir_all("images").map_err(|e| FetchError::IoError(e.to_string()))?;

    for i in 1..=5 {
        println!("Fetching random dog image #{}...", i);

        match fetch_random_dog_image() {
            Ok(dog_image) => {
                let filename = format!("images/dog_image_{}.jpg", i);
                println!("✅ Got image URL: {}", dog_image.message);

                match download_image(&dog_image.message, &filename) {
                    Ok(_) => println!("💾 Saved to: {}", filename),
                    Err(e) => println!("❌ Failed to download: {}", e),
                }
            }
            Err(e) => println!("❌ Error fetching image: {}", e),
        }

        println!();
    }

    Ok(())
}
