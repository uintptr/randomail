use std::fmt::Display;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use tabled::{
    Table,
    settings::{Rotate, Style},
};

use randomail_api::{
    cf_email::{
        add_email_route, delete_email_route, list_email_routes, rename_email_route,
        update_email_route,
    },
    config::RMConfig,
};

#[derive(Args)]
struct ConfigArgs {
    /// Account ID
    #[arg(long, short = 'i')]
    account_id: Option<String>,

    /// API Token
    #[arg(long, short)]
    token: Option<String>,

    /// Destination Email Address
    #[arg(long, short)]
    email: Option<String>,

    /// Email Domain
    #[arg(long, short)]
    domain: Option<String>,
}

#[derive(Args)]
struct AddArgs {
    /// alias
    #[arg(long, short)]
    alias: String,

    /// description
    #[arg(long, short)]
    description: String,
}

#[derive(Args)]
struct DeleteArgs {
    /// email id
    email_id: String,
}

#[derive(Args)]
struct ToggleArgs {
    /// email id
    email_id: String,
}

#[derive(Args)]
struct RenameArgs {
    /// email id
    #[arg(long, short)]
    email_id: String,

    /// new name
    #[arg(long, short)]
    name: String,
}

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),
    #[command(alias = "ls")]
    List,
    Add(AddArgs),
    #[command(alias = "rm")]
    Delete(DeleteArgs),
    Disable(ToggleArgs),
    Enable(ToggleArgs),
    Rename(RenameArgs),
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

fn command_config(args: &ConfigArgs) -> Result<()> {
    let mut data = RMConfig::soft_load().unwrap_or_default();

    data.update(
        args.account_id.clone(),
        args.token.clone(),
        args.email.clone(),
        args.domain.clone(),
    )?;

    let conf = RMConfig::soft_load()?;

    let mut table = Table::new(vec![conf]);
    table.with(Style::modern_rounded());
    table.with(Rotate::Left);

    println!("{table}");

    Ok(())
}

fn command_list() -> Result<()> {
    let conf = RMConfig::load()?;

    let routes = list_email_routes(&conf.zone_id, &conf.token)?;

    let mut table = Table::new(&routes);
    table.with(Style::modern_rounded());

    println!("{table}");

    Ok(())
}

fn command_add<A, D>(alias: &A, description: D) -> Result<()>
where
    A: Into<String> + Display,
    D: Into<String> + Display,
{
    let config = RMConfig::load()?;

    let email_alias = format!("{alias}@{}", config.zone);

    add_email_route(
        config.zone_id,
        description,
        email_alias,
        config.destination_email,
        config.token,
    )
}

fn command_del<I>(email_id: I) -> Result<()>
where
    I: AsRef<str> + Display,
{
    let config = RMConfig::load()?;

    delete_email_route(config.zone_id, email_id, config.token)
}

fn command_disable<I>(email_id: I) -> Result<()>
where
    I: AsRef<str> + Display,
{
    let config = RMConfig::load()?;

    update_email_route(config.zone_id, email_id, config.token, false)
}

fn command_enable<I>(email_id: I) -> Result<()>
where
    I: AsRef<str> + Display,
{
    let config = RMConfig::load()?;

    update_email_route(config.zone_id, email_id, config.token, true)
}

fn command_rename(args: &RenameArgs) -> Result<()> {
    let config = RMConfig::load()?;
    rename_email_route(config.zone_id, &args.email_id, config.token, &args.name)
}

fn main() -> Result<()> {
    let args = UserArgs::parse();

    init_logging(args.verbose);

    match args.command {
        Commands::Config(a) => command_config(&a),
        Commands::List => command_list(),
        Commands::Add(a) => command_add(&a.alias, a.description),
        Commands::Delete(a) => command_del(a.email_id),
        Commands::Disable(a) => command_disable(a.email_id),
        Commands::Enable(a) => command_enable(a.email_id),
        Commands::Rename(a) => command_rename(&a),
    }
}
