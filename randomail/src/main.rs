use std::fmt::Display;

use anyhow::{Result, bail};
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
    /// Cloudflare account ID
    #[arg(long, short = 'i')]
    account_id: Option<String>,

    /// Cloudflare API token
    #[arg(long, short)]
    token: Option<String>,

    /// Destination email address that aliases forward to
    #[arg(long, short)]
    email: Option<String>,

    /// Domain to create email aliases under
    #[arg(long, short)]
    domain: Option<String>,
}

#[derive(Args)]
struct AddArgs {
    /// Name for the new email alias (e.g. "shopping" for shopping@domain.com)
    #[arg(long, short)]
    alias: String,

    /// Human-readable description for the alias
    #[arg(long, short)]
    description: String,
}

#[derive(Args)]
struct RemoveArgs {
    /// Email alias to remove (e.g. shopping@domain.com)
    email: String,
}

#[derive(Args)]
struct ToggleArgs {
    /// Email alias to enable or disable
    email: String,
}

#[derive(Args)]
struct RenameArgs {
    /// Email alias to rename
    #[arg(long, short)]
    email: String,

    /// New description for the alias
    #[arg(long, short)]
    name: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Set or update Cloudflare configuration
    Config(ConfigArgs),
    #[command(alias = "ls")]
    /// List all email aliases and their status
    List,
    /// Create a new email alias
    Add(AddArgs),
    /// Delete an email alias permanently
    #[command(alias = "rm")]
    Remove(RemoveArgs),
    /// Disable an email alias without deleting it
    Disable(ToggleArgs),
    /// Re-enable a previously disabled email alias
    Enable(ToggleArgs),
    /// Update the description of an email alias
    Rename(RenameArgs),
}

#[derive(Parser)]
#[command(version)]
struct UserArgs {
    /// Enable verbose logging output
    #[arg(long, short)]
    verbose: bool,

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

async fn get_email_id(config: &RMConfig, email: &str) -> Result<String> {
    let routes = list_email_routes(&config.zone_id, &config.token).await?;

    for r in routes {
        if r.email_alias.eq(email) {
            return Ok(r.id);
        }
    }

    bail!("email id not found for {email}")
}

async fn command_config(args: &ConfigArgs) -> Result<()> {
    let mut data = RMConfig::soft_load().unwrap_or_default();

    data.update(
        args.account_id.clone(),
        args.token.clone(),
        args.email.clone(),
        args.domain.clone(),
    )
    .await?;

    let conf = RMConfig::soft_load()?;

    let mut table = Table::new(vec![conf]);
    table.with(Style::modern_rounded());
    table.with(Rotate::Left);

    println!("{table}");

    Ok(())
}

async fn command_list() -> Result<()> {
    let conf = RMConfig::load()?;

    let routes = list_email_routes(&conf.zone_id, &conf.token).await?;

    let mut table = Table::new(&routes);
    table.with(Style::modern_rounded());

    println!("{table}");

    Ok(())
}

async fn command_add<A, D>(alias: &A, description: D) -> Result<()>
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
    .await
}

async fn command_rem<I>(email: I) -> Result<()>
where
    I: AsRef<str> + Display,
{
    let config = RMConfig::load()?;

    let email_id = get_email_id(&config, email.as_ref()).await?;

    delete_email_route(config.zone_id, email_id, config.token).await
}

async fn command_disable<I>(email: I) -> Result<()>
where
    I: AsRef<str> + Display,
{
    let config = RMConfig::load()?;

    let email_id = get_email_id(&config, email.as_ref()).await?;

    update_email_route(config.zone_id, email_id, config.token, false).await
}

async fn command_enable<I>(email: I) -> Result<()>
where
    I: AsRef<str> + Display,
{
    let config = RMConfig::load()?;

    let email_id = get_email_id(&config, email.as_ref()).await?;

    update_email_route(config.zone_id, email_id, config.token, true).await
}

async fn command_rename(args: &RenameArgs) -> Result<()> {
    let config = RMConfig::load()?;
    rename_email_route(config.zone_id, &args.email, config.token, &args.name).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = UserArgs::parse();

    init_logging(args.verbose);

    match args.command {
        Commands::Config(a) => command_config(&a).await,
        Commands::List => command_list().await,
        Commands::Add(a) => command_add(&a.alias, a.description).await,
        Commands::Remove(a) => command_rem(a.email).await,
        Commands::Disable(a) => command_disable(a.email).await,
        Commands::Enable(a) => command_enable(a.email).await,
        Commands::Rename(a) => command_rename(&a).await,
    }
}
