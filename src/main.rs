use anyhow::Result;

use clap::{Args, Parser, Subcommand};
use log::LevelFilter;

use crate::{
    config::{ConfigArgs, command_config},
    routes::{command_add, command_list},
};

mod cf_destination;
mod cf_email;
mod cf_zone;
mod common;
mod config;
mod http;
mod routes;

#[derive(Args)]
pub struct AddArgs {
    /// alias
    #[arg(long, short)]
    alias: String,

    /// description
    #[arg(long, short)]
    description: String,
}

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),
    List,
    Add(AddArgs),
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
        Commands::Add(a) => command_add(a.alias, a.description),
    }
}
