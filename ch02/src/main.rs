use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup logger configuration
    env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let settings = Settings::get_config()?;
    for page in 1..=settings.total_page {
        let index_html = scrape_index(&page.to_string()).await?;
        let detail_urls = parse_index(&index_html).await?;
        for detail_url in detail_urls {
            info!("detail url {:?}", &detail_url);
            let detail_html = scrape_detail(&detail_url);
            let data = parse_detail(&detail_html.await).await;

            info!("Get detail data {:#?}", data);
        }
        println!();
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Settings {
    pub base_url: String,
    pub total_page: u32,
}

impl Settings {
    pub fn get_config() -> Result<Settings, Box<dyn Error>> {
        let settings = toml::from_str(&fs::read_to_string("settings.toml")?)?;
        Ok(settings)
    }
}

async fn scrape_page(url: &str) -> Result<String, Box<dyn Error>> {
    info!("scraping {url}");
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        Ok(resp.text().await?)
    } else {
        error!("error occurred while scraping {url}");
        panic!("")
    }
}

async fn scrape_index(page: &str) -> Result<String, Box<dyn Error>> {
    let base_url = Settings::get_config()?.base_url;
    let index_url = format!("{base_url}/page/{page}");
    Ok(scrape_page(&index_url).await.unwrap())
}

async fn parse_index(html: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let re = Regex::new(r#"<a.*?href="(.*?)".*?class="name"#).unwrap();
    let mut detail_urls: Vec<String> = Vec::new();

    let base_url = Settings::get_config()?.base_url;
    for (_, [item]) in re.captures_iter(html).map(|m| m.extract()) {
        detail_urls.push(format!("{base_url}{}", item));
    }

    Ok(detail_urls)
}

async fn scrape_detail(url: &str) -> String {
    scrape_page(url).await.unwrap()
}

async fn parse_detail(html: &str) -> FilmInfo {
    let cover_re = Regex::new(r#"(?ms)class="item.*?<img.*?src="(.*?)@.*?class="cover">"#).unwrap();
    let name_re = Regex::new(r#"(?ms)<h2.*?>(.*?)</h2>"#).unwrap();
    let categories_re =
        Regex::new(r#"(?ms)<button.*?category.*?<span>(.*?)</span>.*?</button>"#).unwrap();
    let published_at_re = Regex::new(r"(?ms)(\d{4}-\d{2}-\d{2})\s?上映").unwrap();
    // let drama_re = Regex::new(r#"(?ms)<div.*?drama.*?>.*?<p>.*?(.*?)</p>"#).unwrap();
    let drama_re = Regex::new(r#"(?ms)<h3.*?>.*?</h3>.*?<p.*?>(.*?)</p>"#).unwrap();
    let score_re = Regex::new(r#"(?ms)<p.*?score.*?>(.*?)</p>"#).unwrap();

    let cover = cover_re.captures(html).unwrap().get(1).unwrap().as_str();
    let name = name_re.captures(html).unwrap().get(1).unwrap().as_str();
    let categories: Vec<String> = categories_re
        .captures_iter(html)
        .map(|caps| {
            let (_, [cate]) = caps.extract();
            cate.to_string()
        })
        .collect();

    let published_at = if published_at_re.is_match(html) {
        published_at_re
            .captures(html)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
    } else {
        ""
    };
    let drama = drama_re.captures(html).unwrap().get(1).unwrap().as_str();
    let score = score_re.captures(html).unwrap().get(1).unwrap().as_str();

    FilmInfo {
        cover: cover.to_string(),
        name: name.to_string(),
        categories,
        published_at: published_at.to_string(),
        drama: drama.trim().to_string(),
        score: score.trim().to_string(),
    }
}

#[derive(Debug, Default)]
pub struct FilmInfo {
    pub cover: String,
    pub name: String,
    pub categories: Vec<String>,
    pub published_at: String,
    pub drama: String,
    pub score: String,
}
