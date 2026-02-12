use anyhow::Result;

use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::config::{ConfigArgs, command_config};

mod common;
mod config;
//mod dest;
mod cfapi;
mod http;
//mod responses;

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),
}

#[derive(Parser)]
struct UserArgs {
    /// verbose
    #[arg(long, short)]
    verbose: bool,

    /// Command
    #[command(subcommand)]
    command: Commands,
}

/*
fn command_list() -> Result<()> {
    let _default = default_destinaton()?;

    let mail_token = CFConfig::load()?;

    let url = format!("{CF_API_URL}/zones/fb4a35a23fc2a6f712a6f062d138b2ea/email/routing/rules");

    let data = issue_get(&url, &mail_token.token)?;

    println!("{data}");

    Ok(())
}
*/

fn init_logging(verbose: bool) {
    let level = if verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    env_logger::builder().filter_level(level).init();
}

fn main() -> Result<()> {
    let args = UserArgs::parse();

    init_logging(args.verbose);

    match args.command {
        Commands::Config(a) => command_config(&a),
    }
}
