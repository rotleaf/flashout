pub mod account {
    use std::{env, error::Error, sync::Arc, time::Duration};

    use colored::Colorize;
    use headless_chrome::{Browser, Tab};

    use crate::{bot::login, utils::screenshot};

    pub async fn redeem_airtime(
        tab: Arc<Tab>,
        amount: i32,
        phone: String,
        op: i64,
        browser: Browser,
    ) -> Result<(), Box<dyn Error>> {
        let url: &str = "https://flashout.io/account/rewards";
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;

        print!(" > {}", "checking auth status".bold().yellow());
        std::thread::sleep(Duration::from_secs(2));
        if tab
            .wait_for_element_with_custom_timeout(
                ".v-toolbar-title__placeholder",
                Duration::from_secs(3),
            )
            .is_ok()
        {
            println!(" [{}]", "ERR".bold().red());
            println!(" > not logged in-[{}]", "loggin in".bold().yellow());
            let _ = login::login::init(tab.clone(), browser).await;
            let url: &str = "https://flashout.io/account/rewards";
            tab.navigate_to(url)?;
            tab.wait_until_navigated()?;
        }
        println!(" [{}]", "OK".bold().green());

        let mut am: i32 = 2; // amount- by default its the first value which is 2

        if amount == 5 {
            am = 2;
        } else if amount == 10 {
            am = 3;
        } else if amount == 20 {
            am = 4;
        } else if amount == 35 {
            am = 5;
        } else if amount == 50 {
            am = 6;
        } else {
            println!(" > {}", "invalid amount".bold().red());
        }

        let balance: headless_chrome::Element = tab.wait_for_element(".text-h3")?; // you can see the balance
        println!(" * Your balance is {}", balance.get_inner_text()?);

        let currency: String = env::var("CURRENCY").expect("CURRENCY not set yet");
        let redeem_button: headless_chrome::Element = tab.find_element(".bg-primary").unwrap();
        println!(
            " > redeeming {} {} to {}",
            amount.to_string().bold().green(),
            currency.bold().green(),
            format!("+{}", phone).bold()
        );
        redeem_button.click().unwrap();
        let _ = tab.wait_until_navigated().unwrap();
        tab.wait_for_element("label.v-label:nth-child(2)")?;

        std::thread::sleep(Duration::from_secs(2));
        tab.find_element(".v-container")?;

        let phone_element: headless_chrome::Element = tab
            .find_element(
                "div.v-input:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(3)",
            )
            .unwrap();
        // click phone input
        phone_element.click().unwrap();
        // type the phone number
        phone_element.type_into(&format!("+{}", phone)).unwrap();

        let network_element: headless_chrome::Element = tab
            .wait_for_element("div.v-input:nth-child(2) > div:nth-child(1)")
            .unwrap();
        // click network input
        network_element.click().unwrap();
        // click one item, :2 - saf:3-Airtel: 4-Telkom ,
        tab.find_element(&format!(".v-list-item:nth-child({})", op))
            .unwrap()
            .click()
            .unwrap();

        let amount_element: headless_chrome::Element = tab
            .wait_for_element("div.v-input:nth-child(3) > div:nth-child(1) > div:nth-child(1)")
            .unwrap();
        // click the balance input
        amount_element.click().unwrap();
        // select one,

        // let items = tab.find_element(".v-overlay-container").unwrap().get_content().unwrap();
        // (\:)/)2 - 5ksh | 3 - 10 | 4 - 20 | 5 - 35 | 6 - 50
        tab.find_element(&format!(".v-list-item:nth-child({})", am))
            .unwrap()
            .click()
            .unwrap();
        // confirm the number is yours and you would like to continue
        let check_box: headless_chrome::Element =
            tab.find_element("label.v-label:nth-child(2)").unwrap();
        check_box.click().unwrap();

        // initiate withdrawal
        let redeem_button: headless_chrome::Element =
            tab.find_element("button.v-btn:nth-child(5)").unwrap();
        redeem_button.click().unwrap();

        match tab.wait_for_element("div.v-container:nth-child(3)") {
            Ok(_) => {
                println!(
                    " > credit delivered-[{}{}]",
                    amount.to_string().bold(),
                    currency.to_string().bold()
                );
            }
            Err(err) => {
                println!(
                    " * {}-[{}]",
                    "error".bold().red(),
                    err.to_string().bold().red()
                );
            }
        }

        screenshot::screenshot::get_png(tab, Some("final.png".to_string()))?;

        Ok(())
    }
}
