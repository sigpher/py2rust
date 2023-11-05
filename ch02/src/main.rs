use log::{error, info};
use regex::Regex;
use std::{env, error::Error};

const BASE_URL: &str = "https://ssr1.scrape.center";
const TOTAL_PAGE: u8 = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    for page in 1..=TOTAL_PAGE {
        let index_html = scrape_index(&page.to_string()).await?;
        let detail_urls = parse_index(&index_html).await;
        for detail_url in &detail_urls {
            info!("detail url {:?}", &detail_url);
            let detail_huml = scrape_detail(detail_url).await;
            let data = parse_detail(&detail_huml).await;

            info!("Get detail data {:#?}", data);
        }
        println!();
    }

    Ok(())
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
    let index_url = format!("{BASE_URL}/page/{page}");
    Ok(scrape_page(&index_url).await.unwrap())
}

async fn parse_index(html: &str) -> Vec<String> {
    let re = Regex::new(r#"<a.*?href="(.*?)".*?class="name"#).unwrap();
    let mut detail_urls: Vec<String> = Vec::new();

    for (_, [item]) in re.captures_iter(html).map(|m| m.extract()) {
        detail_urls.push(format!("{BASE_URL}{}", item));
    }

    detail_urls
}

async fn scrape_detail(url: &str) -> String {
    scrape_page(url).await.unwrap()
}

async fn parse_detail(html: &str) -> FilmInfo {
    let cover_re = Regex::new(r#"(?ms)class="item.*?<img.*?src="(.*?)".*?class="cover">"#).unwrap();
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
