use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{env, error::Error, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup logger configuration
    env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let settings = Settings::get_config()?;
    let pool = SqlitePool::connect("test.db").await?;

    let mut fs = Vec::new();
    for page in 1..=settings.total_page {
        let index_html = scrape_index(&page.to_string()).await?;
        let detail_urls = parse_index(&index_html).await?;
        for detail_url in detail_urls {
            let pool = pool.clone();
            let h = tokio::spawn(async move {
                // info!("detail url {:?}", &detail_url);
                scrape_detail(&detail_url).await;
                let detail_html = scrape_detail(&detail_url).await;
                let data = parse_detail(&detail_html);

                // info!("Get detail data {:#?}", data);
                save_data(pool, data.await).await.unwrap();
            });
            fs.push(h);
        }
    }
    for f in fs {
        let _handle = tokio::join!(f);
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

// scrape_page: return the html body text of the page
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
        cover,
        name,
        // categories,
        published_at,
        drama: drama.trim(),
        score: score.trim(),
    }
}

#[derive(Debug, Default)]
pub struct FilmInfo<'a> {
    pub cover: &'a str,
    pub name: &'a str,
    // pub categories: Vec<String>,
    pub published_at: &'a str,
    pub drama: &'a str,
    pub score: &'a str,
}

async fn save_data<'a>(pool: SqlitePool, file_info: FilmInfo<'a>) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let sql = r#"
INSERT INTO FILES (cover,name,published_at,drama,score) VALUES (?,?,?,?,?)
        "#;
    // let id = sqlx::query(sql)
    sqlx::query(sql)
        .bind(file_info.cover)
        .bind(file_info.name)
        // .bind(file_info.categories
        .bind(file_info.published_at)
        .bind(file_info.drama)
        .bind(file_info.score)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();
    info!("saving data: {:#?}", &file_info);

    Ok(())
}

