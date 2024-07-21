pub mod init {
    use crate::close_tabs;
    use crate::login;
    use crate::redeem_airtime;
    use crate::Args;
    use colored::Colorize;
    use headless_chrome::*;
    use std::env;
    use std::error::Error;
    use std::ffi::OsStr;
    use std::path::PathBuf;
    use std::sync::Arc;

    /// begin bot, the entry function
    pub async fn begin(p_args: Args) -> Result<(), Box<dyn Error>> {
        let ua: String = env::var("USER_AGENT").expect("must set USER_AGENT");
        let user_agent: String = format!("--user-agent={}", ua);

        let mut args: Vec<String> = vec!["--no-sandbox".to_string(), user_agent.to_string()];

        if let Some(proxy) = &p_args.proxy {
            args.push(format!("--proxy-server={}", proxy));
        }

        let os_args: Vec<&OsStr> = args.iter().map(AsRef::as_ref).collect();

        let launch_options: headless_chrome::LaunchOptions = LaunchOptionsBuilder::default()
            .args(os_args)
            .headless(!p_args.user_interface)
            .user_data_dir(Some(PathBuf::from("/tmp")))
            .disable_default_args(true)
            .build()
            .unwrap();

        let browser: Browser = Browser::new(launch_options)?;
        let tab: Arc<headless_chrome::Tab> = browser.new_tab()?;
        // tab.enable_fetch(None, None)?;
        let _ = login::init(tab.clone(), browser.clone());
        if p_args.action.as_deref().map(str::to_lowercase) == Some("withdraw".to_string())
            || p_args.action.as_deref().map(str::to_lowercase) == Some("redeem".to_string())
        {
            let op: i64 = match p_args.network.as_deref().map(str::to_lowercase).as_deref() {
                Some("airtel") => 3,
                Some("safaricom") | Some("saf") => 2,
                _ => {
                    println!(" * Invalid Operator name. Options are: safaricom or saf, airtel");
                    return Ok(());
                }
            };

            let _ = redeem_airtime(
                tab.clone(),
                p_args.credit_amount.unwrap(),
                p_args.phone.unwrap(),
                op,
                &browser,
            )
            .await;
        } else {
            println!(
                " * {}",
                "provide an action, [withdraw, redeem]".bold().red()
            );
            close_tabs(browser)?;
        }
        Ok(())
    }
}
