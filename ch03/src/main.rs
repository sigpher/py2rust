#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let url = "https://ssr3.scrape.center/page/1";
    let resp = client
        .get(url)
        // .basic_auth("admin", Some("admin"))
        .basic_auth("admin", Some("admin"))
        .send()
        .await
        .unwrap();

    println!("{}", resp.text().await.unwrap());
}
