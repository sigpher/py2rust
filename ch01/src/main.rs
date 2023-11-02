use std::{collections::HashMap, fs::File, io::Write, time::Duration};

use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let url = "https://httpbin.org/ip";
    let resp = client.get(url).send().await?;
    let status = resp.status();
    let headers = resp.headers();
    let header_server = resp.headers().get("server").unwrap();
    println!("Status: {}", status);
    println!("Headers: {:?}", headers);
    println!("Header server: {:?}", header_server);
    println!("----------------------");

    let mut data = HashMap::new();
    data.insert("name".to_string(), "germey".to_string());
    data.insert("encoding".to_string(), "utf-8".to_string());
    let resp = client
        .get("https://ssr3.scrape.center/")
        .timeout(Duration::from_secs(10))
        .basic_auth("admin", Some("admin"))
        .query(&data)
        .send()
        .await?
        .text_with_charset("utf-8")
        .await?;
    println!("{}", resp);
    println!("----------------------");

    // let url = "https://jsonplaceholder.typicode.com/albums/2/photos";
    // client.get(url).send().await?.cookies().for_each(|x| {j
    //     println!("{:?}", x);
    // });

    let doc = Html::parse_document(&resp);
    let selector = Selector::parse(".m-b-sm").unwrap();

    for el in doc.select(&selector) {
        println!("title:{}", el.inner_html());
    }
    let url = Url::parse("https://httpbin.org:80/post?name=choi&age=20");
    // url.unwrap()
    //     .query()
    //     .unwrap()
    //     .splitn(2, '&')
    //     .for_each(|x| x.splitn(2, '=').for_each(|x| println!("{x}")));

    println!("{:?}", url.unwrap().port());
    let url = Url::parse("https://www.baidu.com/").unwrap();
    let url = &url.join("/choi").unwrap();
    println!("{:?}", url.as_str());
    //
    // let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    //
    // println!("{:?}", robots_txt);

    let resp = reqwest::get("https://ssr1.scrape.center/").await?;
    let body = resp.text().await?;
    let re = Regex::new(r"<h2.*?>(.*?)</h2>").unwrap();

    for (_, [item]) in re.captures_iter(&body).map(|x| x.extract()) {
        println!("{:?}", item);
    }
    println!("----------------");

    let url = "https://scrape.center/favicon.ico";

    let resp = client.get(url).send().await?;
    let favicon = resp.bytes().await?;

    let mut file = File::options()
        // .create_new(true)
        .create(true)
        .write(true)
        .truncate(true)
        .open("fav.ico")
        .unwrap();
    file.write_all(&favicon).unwrap();
    file.flush().unwrap();
    Ok(())
}
