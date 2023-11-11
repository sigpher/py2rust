use std::{thread::sleep, time::Duration};

use playwright::Playwright;
#[tokio::main]
async fn main() -> std::result::Result<(), playwright::Error> {
    let pw = Playwright::initialize().await?;
    // pw.prepare()?;
    // let browser = pw.chromium().launcher().headless(true).launch().await?;
    let browser = pw.chromium().launcher().launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://ssr1.scrape.center/")
        .goto()
        .await?;
    sleep(Duration::from_secs(10));
    Ok(())
}
