pub mod login {
    use colored::Colorize;
    use headless_chrome::{Browser, Tab};
    use std::{env, error::Error, process, sync::Arc, thread, time::Duration};

    pub async fn init(tab: Arc<Tab>, browser: Browser) -> Result<(), Box<dyn Error>> {
        let email: String = env::var("EMAIL").expect("set EMAIL");
        let password: String = env::var("PASSWORD").expect("set PASSWORD");
        tab.navigate_to("https://flashout.io/login")?;

        let _ = tab.wait_until_navigated();
        thread::sleep(Duration::from_secs(1));

        let emailfield: headless_chrome::Element = tab.find_element("#email").unwrap();
        emailfield.click().unwrap();
        emailfield.type_into(&email).unwrap();

        let passfield: headless_chrome::Element = tab.find_element("#password").unwrap();
        passfield.click().unwrap();
        passfield.type_into(&password).unwrap();

        let btn: headless_chrome::Element = tab.find_element("button.v-btn:nth-child(3)").unwrap();
        btn.click()?;
        let _ = tab.wait_until_navigated();

        thread::sleep(Duration::from_secs(2));
        if tab.wait_for_element("div.v-alert:nth-child(3)").is_ok() {
            println!(
                " * {}-[{}]",
                "check credentials".bold().red(),
                "login failed".bold().yellow()
            );
            // close all tabs
            let tabs: &Arc<std::sync::Mutex<Vec<Arc<Tab>>>> = browser.get_tabs();
            let locked_tabs: std::sync::MutexGuard<Vec<Arc<Tab>>> =
                tabs.lock().expect("failed to lock tabs mutex");
            for tab in locked_tabs.iter() {
                tab.close_target()?;
            }
            process::exit(0);
        }
        let _ = tab.wait_for_element("div.v-card");
        println!(" - {}", "logged in".bold().green());

        Ok(())
    }
}
