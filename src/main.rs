use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, panic};

use bot::account::account::redeem_airtime;
use bot::login::login;
use clap::{command, ArgAction, Parser};
use colored::Colorize;
use dotenv::dotenv;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use utils::browser::browser_utils::close_tabs;

pub mod bot;
pub mod utils;

#[derive(Parser, Debug)]
#[command(version="0.1.7-beta", about="flashout autopilot", long_about = None)]
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

#[tokio::main]
async fn main() {
    panic::set_hook(Box::new(|panic_info: &panic::PanicInfo| {
        let message: String = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic reason".to_string()
        };

        println!(" [{}] {}", "PANIC".bold().red(), message.bold().white());
    }));

    dotenv().ok();
    let args: Args = Args::parse();
    let _ = begin(args).await;
}
