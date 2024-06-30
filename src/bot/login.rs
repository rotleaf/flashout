pub mod login {
    use std::{env, error::Error, sync::Arc, thread, time::Duration};

    use headless_chrome::Tab;

    pub async fn init(tab: Arc<Tab>) -> Result<(), Box<dyn Error>> {
        let email = env::var("EMAIL").expect("set EMAIL");
        let password = env::var("PASSWORD").expect("set PASSWORD");
        tab.navigate_to("https://flashout.io/login")?;

        let _ = tab.wait_until_navigated();
        thread::sleep(Duration::from_secs(1));

        let emailfield = tab.find_element("#email").unwrap();
        emailfield.click().unwrap();
        emailfield.type_into(&email).unwrap();

        let passfield = tab.find_element("#password").unwrap();
        passfield.click().unwrap();
        passfield.type_into(&password).unwrap();

        let btn = tab.find_element("button.v-btn:nth-child(3)").unwrap();
        btn.click()?;
        let _ = tab.wait_until_navigated();

        thread::sleep(Duration::from_secs(2));
        let _ = tab.wait_for_element("div.v-card");
        println!("logged in");

        Ok(())
    }
}
