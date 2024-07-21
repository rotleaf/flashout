use clap::{command, ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version = "0.1.10", about = "flashout autopilot", author = "Mbithi")]
/// flashout program cmdline arguments
pub struct Args {
    /// action to perform, [withdraw or redeem, tasks]
    #[arg(short, long)]
    pub action: Option<String>,
    /// phone number to withdraw to
    #[arg(short, long)]
    pub phone: Option<String>,
    /// amount to withdraw, 10, 20, 35, 50
    #[arg(short, long)]
    pub credit_amount: Option<i32>,
    /// network, [airtel, safaricon(saf)]
    #[arg(short, long)]
    pub network: Option<String>,
    /// include ui, run non headless mode (not recommended)
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub user_interface: bool,
    /// proxy server
    #[arg(long)]
    pub proxy: Option<String>,
}
