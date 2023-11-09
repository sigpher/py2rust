use std::error::Error;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://spa1.scrape.center/api/movie/?limit=10&offset=";
    let mut urls = vec![];
    for i in 1..=10 {
        let offset = format!("{base_url}{}", (i - 1) * 10);
        urls.push(offset)
    }
    let client = reqwest::Client::new();
    for url in urls {
        let resp = client.get(url).send().await?;
        let resp_data = resp.json::<ResponseData>().await?;
        println!("{:?}", resp_data.results);
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Film {
    pub id: u32,
    pub name: String,
    pub alias: String,
    pub cover: String,
    pub categories: Vec<String>,
    pub published_at: Option<String>,
    pub minute: u32,
    pub score: f32,
    pub regions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    count: u32,
    results: Vec<Film>,
}
