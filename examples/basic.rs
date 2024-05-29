//! ## Setup
//!
//! This example assumes you have geckodriver or chromedriver listening at port 4444.
//!
//! You can start the webdriver instance by:
//!
//! ### geckodriver
//!
//! ```text
//! geckodriver --port 4444
//! ```
//!
//! ### chromedriver
//!
//! ```text
//! chromedriver --port=4444
//! ```
//!
//! ## To Run
//!
//! ```
//! cargo run --example basic
//! ```

use fantoccini::{ClientBuilder, Locator};
use serde::Serialize;
use serde_json::Map;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
pub struct LambdaTestCapabilities {
    //#[serde(rename = "build")]
    //pub build_id: String,
    #[serde(rename = "accessKey")]
    pub access_key: String,
    //pub name: &'static str,
    pub username: String,
    #[serde(rename = "platformName")]
    pub platform: String,
    pub tunnel: bool,
    pub project: &'static str,
    //pub console: bool,
    pub w3c: bool,
}

const USER: &str = "";
const PASS: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let lt_options = LambdaTestCapabilities {
        username: USER.to_owned(),
        access_key: PASS.to_owned(),
        tunnel: false,
        w3c: true,
        platform: "Windows 10".to_owned(),
        project: "Untitled",
    };

    let mut capabilities = Map::new();
    capabilities.insert("LT:Options".to_owned(), serde_json::to_value(lt_options).unwrap());
    capabilities.insert("browserName".to_owned(), serde_json::to_value("chrome").unwrap());
    capabilities.insert("browserVersion".to_owned(), serde_json::to_value("125").unwrap());


    let url = format!("https://{USER}:{PASS}@hub.lambdatest.com/wd/hub/");

    let client = ClientBuilder::native()
        .capabilities(capabilities)
        .connect(&url)
        .await
        .expect("client to connect");

    // Go to the Rust website.
    client.goto("https://www.rust-lang.org/").await?;

    // Click the "Get Started" button.
    let button = client
        .wait()
        .for_element(Locator::Css(
            r#"a.button-download[href="/learn/get-started"]"#,
        ))
        .await?;
    button.click().await?;

    // Click the "Try Rust Without Installing" button (using XPath this time).
    let button = r#"//a[@class="button button-secondary" and @href="https://play.rust-lang.org/"]"#;
    let button = client.wait().for_element(Locator::XPath(button)).await?;
    button.click().await?;

    // Find the big textarea.
    let code_area = client
        .wait()
        .for_element(Locator::Css(".ace_text-input"))
        .await?;

    // And write in some code.
    code_area.send_keys("// Hello from Fantoccini\n").await?;

    // Now, let's run it!
    let button = r#"//button[.='Run']"#;
    let button = client.wait().for_element(Locator::XPath(button)).await?;
    button.click().await?;

    // Let the user marvel at what we achieved.
    sleep(Duration::from_millis(6000)).await;
    // Then close the browser window.
    client.close().await?;

    Ok(())
}
