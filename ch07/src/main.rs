use std::time::Duration;

use thirtyfour::{
    prelude::{ElementQueryable, WebDriverError},
    By, DesiredCapabilities, EdgeCapabilities, WebDriver,
};

#[tokio::main]
async fn main() -> Result<(), WebDriverError> {
    // let caps = EdgeCapabilities::new();
    let caps = DesiredCapabilities::chrome();

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let url = "https://spa1.scrape.center/";
    driver.goto(url).await?;
    let check = driver.find(By::Css(".m-b-sm")).await?;
    println!("{:?}", check);
    driver.quit().await?;

    Ok(())
}

