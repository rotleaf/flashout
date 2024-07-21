use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use std::error::Error;
use std::panic;
use utils::browser::browser_utils::close_tabs;

pub mod bot;
pub mod utils;

use bot::account::account::redeem_airtime;
use bot::args::Args;
use bot::init::init::begin;
use bot::login::login;
use utils::counter::counter::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    panic::set_hook(Box::new(|panic_info: &panic::PanicInfo| {
        let message: String = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "i panicked, i dont know why though".to_string()
        };

        println!(" [{}] {}", "PANIC".bold().red(), message.bold().white());
    }));

    dotenv().ok();

    loop {
        let args: Args = Args::parse();
        let _ = begin(args).await;
        // adjust this to your liking, the minimum should be 10 minutes
        let _ = init(10).await?;
    }
}
