pub mod screenshot {
    use std::{error::Error, sync::Arc};

    use headless_chrome::{protocol::cdp::Page, Tab};

    pub fn get_png(tab: Arc<Tab>, name: Option<String>) -> Result<(), Box<dyn Error>> {
        let png_data =
            tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
        // Save the screenshot to disc
        std::fs::write(name.unwrap_or("screenshot.png".to_owned()), png_data)?;
        Ok(())
    }
}
