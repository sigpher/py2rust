use std::{thread::sleep, time::Duration};

use serde_json::json;
use thirtyfour::{extensions::cdp::ChromeDevTools, prelude::*};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    //anti shield
    /////////////////////////////
    let dev_tools = ChromeDevTools::new(driver.handle.clone());
    dev_tools
        .execute_cdp_with_params(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({"source": "Object.defineProperty(navigator, 'webdriver', {get:()=>undefined})" }),
        )
        .await?;
    /////////////////////////////
    driver.goto("https://ssr1.scrape.center/").await?;
    sleep(Duration::from_secs(5));
    driver.quit().await?;

    Ok(())
}
