use std::{collections::HashMap, time::Duration};

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

    let url = "https://jsonplaceholder.typicode.com/albums/2/photos";
    client.get(url).send().await?.cookies().for_each(|x| {
        println!("{:?}", x);
    });

    Ok(())
}
