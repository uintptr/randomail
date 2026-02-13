use anyhow::{Result, bail};

use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::{
    cfapi::list_email_forward_route,
    config::{CFConfig, ConfigArgs, command_config},
};

mod cfapi;
mod common;
mod config;
mod http;

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),
    List,
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

fn command_list() -> Result<()> {
    let conf = CFConfig::load()?;

    if !conf.ready() {
        bail!("configuration is not ready");
    }

    let routes = list_email_forward_route(&conf.zone_id, &conf.destination_email, &conf.token)?;

    if let Some(email) = routes.matchers.first() {
        println!(
            "id={} name={} email={:?}",
            routes.id, routes.name, email.value
        );
    }

    Ok(())
}

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
        Commands::List => command_list(),
    }
}
