pub mod screenshot {
    use std::{error::Error, sync::Arc};

    use headless_chrome::{protocol::cdp::Page, Tab};

    /// get png screenshot of a page,
    /// add this and parse a tab to it, the screenshot name is optional and defaults to
    /// 'screenshot.png'
    pub fn get_png(tab: Arc<Tab>, name: Option<String>) -> Result<(), Box<dyn Error>> {
        let png_data: Vec<u8> =
            tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
        // Save the screenshot to disc
        std::fs::write(name.unwrap_or("screenshot.png".to_owned()), png_data)?;
        Ok(())
    }
}
