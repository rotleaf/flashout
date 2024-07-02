pub mod browser_utils {
    use std::{error::Error, process, sync::Arc};

    use colored::Colorize;
    use headless_chrome::{Browser, Tab};

    pub fn close_tabs(browser: Browser) -> Result<(), Box<dyn Error>> {
        // close all tabs
        let tabs: &Arc<std::sync::Mutex<Vec<Arc<Tab>>>> = browser.get_tabs();
        let locked_tabs: std::sync::MutexGuard<Vec<Arc<Tab>>> =
            tabs.lock().expect("failed to lock tabs mutex");
        for tab in locked_tabs.iter() {
            tab.close_target()?;
        }
        println!(" * ! tabs closed, [{}]", "Exiting".bold().yellow());
        process::exit(0);
    }
}
