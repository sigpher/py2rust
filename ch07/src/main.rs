use std::{thread::sleep, time::Duration};

use serde_json::json;
use thirtyfour::{extensions::cdp::ChromeDevTools, prelude::*};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_ignore_certificate_errors().unwrap();
    caps.set_headless();
    let mut driver = WebDriver::new("http://localhost:9515", caps).await?;
    //anti shield
    /////////////////////////////
    let dev_tools = ChromeDevTools::new(driver.handle.clone());
    dev_tools
        .execute_cdp_with_params(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({"source": "Object.defineProperty(navigator, 'webdriver', {get:()=>undefined})" }),
        )
        .await?;
    // driver
    //     .set_implicit_wait_timeout(Duration::from_secs(10))
    //     .await?;
    /////////////////////////////
    driver.goto("https://ssr1.scrape.center/").await?;
    // driver.execute("window.open()", vec![]).await?;
    // let handle = driver.window().await?;
    let handle = driver.new_tab().await?;
    let handles = driver.window().await?;
    driver.goto("https://www.baidu.com").await?;
    driver.execute("window.open()", vec![]).await?;
    driver.goto("https://www.taobao.com").await?;
    driver.execute("window.open()", vec![]).await?;
    driver.goto("https://ssr1.scrape.center/").await?;
    // driver.goto("https://www.python.org").await?;

    driver.back().await?;
    driver.back().await?;
    driver.back().await?;
    driver.forward().await?;
    driver.forward().await?;
    driver.forward().await?;
    driver.goto("https://www.zhihu.com/explore").await?;
    let cookies = driver.get_all_cookies().await?;
    println!("cookies: {:?}", cookies);
    sleep(Duration::from_secs(3));
    driver.add_cookie(Cookie::new("name", "choi")).await?;
    driver
        .add_cookie(Cookie::new("domain", "www.zhihu.com"))
        .await?;
    let cookies = driver.get_all_cookies().await?;
    sleep(Duration::from_secs(3));
    println!("cookies: {:?}", cookies);

    driver.quit().await?;
    Ok(())
}
