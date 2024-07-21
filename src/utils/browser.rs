pub mod browser_utils {
    use std::{error::Error, process, sync::Arc};

    use colored::Colorize;
    use headless_chrome::{Browser, Tab};

    /// iterate on all open tabs and close them
    /// normally headless_chrome opens 3 tabs
    pub fn close_tabs(browser: Browser) -> Result<(), Box<dyn Error>> {
        // close all tabs
        // only gets called when you are running a `--user-interface` instance
        let tabs: &Arc<std::sync::Mutex<Vec<Arc<Tab>>>> = browser.get_tabs();
        let locked_tabs: std::sync::MutexGuard<Vec<Arc<Tab>>> =
            tabs.lock().expect("failed to lock tabs mutex");
        for tab in locked_tabs.iter() {
            tab.close_target()?;
        }
        println!(" * tabs closed, [{}]", "Exiting".bold().yellow());
        process::exit(0);
    }
}
