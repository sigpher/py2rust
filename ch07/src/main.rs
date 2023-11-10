use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::edge();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("www.baidu.com").await?;

    // Click the search button.
    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

