
use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, process};

use bot::account::account::redeem_airtime;
use bot::login::login;
use clap::{command, ArgAction, Parser};
use dotenv::dotenv;
use headless_chrome::{Browser, LaunchOptionsBuilder};

pub mod bot;
pub mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// action to perform, [withdraw or redeem, tasks]
    #[arg(short, long)]
    action: Option<String>,

    /// phone number to withdraw to
    #[arg(short, long)]
    phone: Option<String>,

    /// amount to withdraw, 5, 10, 20, 35, 50
    #[arg(short, long)]
    credit_amount: Option<i32>,

    /// network, [airtel, safaricon(saf)]
    #[arg(short, long)]
    network: Option<String>,

    /// include ui, run non headless mode (not recommended)
    #[arg(short, long, action = ArgAction::SetTrue)]
    user_interface: bool,

    /// proxy server
    #[arg(long)]
    proxy: Option<String>,
}

async fn begin(p_args: Args) -> Result<(), Box<dyn Error>> {
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
    if p_args.action == Some("withdraw".to_string())
        || p_args.action == Some("Withdraw".to_string())
        || p_args.action == Some("redeem".to_string())
        || p_args.action == Some("Redeem".to_string())
    {
        let mut op = 0;
        if p_args.network == Some("Airtel".to_string())
            || p_args.network == Some("airtel".to_string())
        {
            // op = 265;
            op = 3;
        } else if p_args.network == Some("Safaricom".to_string())
            || p_args.network == Some("safaricom".to_string())
            || p_args.network == Some("saf".to_string())
            || p_args.network == Some("Saf".to_string())
        {
            // op = 266;
            op = 2;
        } else {
            println!(" * Invalid Operator name. Options are: safaricom or saf, airtel");
        }

        // println!("{}", op);
        // maybe loop here
        let _ = redeem_airtime(
            tab.clone(),
            p_args.credit_amount.unwrap(),
            p_args.phone.unwrap(),
            op,
            browser,
        )
        .await;
    } else {
        println!("provide an action dumbo");
        process::exit(0);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Args = Args::parse();

    let _ = begin(args).await;
}