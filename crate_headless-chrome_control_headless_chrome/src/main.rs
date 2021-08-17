use headless_chrome::{protocol::page::ScreenshotFormat, Browser};

fn browse_wikipedia() -> Result<(), failure::Error> {
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;

    // Navigate to wikipedia
    tab.navigate_to("https://www.wikipedia.org")?;

    // Wait for network/javascript/dom to make the search-box available
    // and click it.
    tab.wait_for_element("input#searchInput")?.click()?;

    // Type in a query and press `Enter`
    tab.type_str("WebKit")?.press_key("Enter")?;

    // We should end up on the WebKit-page once navigated
    tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));

    // Take a screenshot of the entire browser window
    let _jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;

    // Take a screenshot of just the WebKit-Infobox
    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    Ok(())
}

fn main() {
    assert!(browse_wikipedia().is_ok());
}
