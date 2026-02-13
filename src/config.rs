use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result, bail};
use clap::Args;
use log::info;
use serde::{Deserialize, Serialize};
use tabled::{
    Table, Tabled,
    settings::{Rotate, Style},
};

use crate::{cf_destination::destination_address, cf_zone::zone_info, common::PROJECT_NAME};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Args)]
pub struct ConfigArgs {
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

#[derive(Deserialize, Serialize, Default, Tabled)]
pub struct RMConfig {
    pub account_id: String,
    pub token: String,
    pub destination_email: String,
    pub destination_email_id: String,
    pub zone: String,
    pub zone_id: String,
}

fn get_config_file() -> Result<PathBuf> {
    let config_root = dirs::config_dir().context("Unable to find config directory")?;

    let config_dir = config_root.join(PROJECT_NAME);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Unable to create {}", config_dir.display()))?;
    }

    Ok(config_dir.join(CONFIG_FILE_NAME))
}

impl RMConfig {
    fn soft_load() -> Result<Self> {
        let config_file = get_config_file()?;

        let mut fd = fs::OpenOptions::new()
            .read(true)
            .open(&config_file)
            .with_context(|| format!("Unable to open {} for reading", config_file.display()))?;

        let mut data = String::new();

        info!("reading {}", config_file.display());

        fd.read_to_string(&mut data)
            .with_context(|| format!("Unable to read {}", config_file.display()))?;

        let token: Self = serde_json::from_str(&data)
            .with_context(|| format!("Unable to deserialize {}", config_file.display()))?;

        Ok(token)
    }

    fn ready(&self) -> bool {
        let mut ready = true;

        if self.account_id.is_empty() {
            eprintln!("account id is missing from config");
            ready = false
        }

        if self.destination_email_id.is_empty() {
            eprintln!("destination email is missing from config");
            ready = false
        }

        if self.zone_id.is_empty() {
            eprintln!("email domain is missing from config");
            ready = false
        }

        ready
    }

    pub fn save(args: &ConfigArgs) -> Result<()> {
        let config_file = get_config_file()?;

        let mut data = Self::soft_load().unwrap_or_default();

        if let Some(account_id) = &args.account_id {
            data.account_id = account_id.clone()
        }

        if let Some(token) = &args.token {
            data.token = token.clone()
        }

        if let Some(email) = &args.email {
            if data.token.is_empty() {
                bail!("token is missing")
            }

            if data.account_id.is_empty() {
                bail!("account_id is missing")
            }

            let dst = destination_address(&data.account_id, email, &data.token)
                .with_context(|| format!("Unable to get email id for  {email}"))?;

            data.destination_email = email.clone();
            data.destination_email_id = dst.id;
        }

        if let Some(zone) = &args.domain {
            if data.token.is_empty() {
                bail!("token is missing")
            }

            let zinfo = zone_info(zone, &data.token)
                .with_context(|| format!("Unable to get zone info for {zone}"))?;

            data.zone = zone.clone();
            data.zone_id = zinfo.id;
        }

        let encoded_data =
            serde_json::to_string_pretty(&data).context("Unable to serialize data")?;

        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_file)
            .with_context(|| format!("Unable to open {} for writing", config_file.display()))?;

        info!("writing {}", config_file.display());

        fd.write_all(encoded_data.as_bytes())
            .with_context(|| format!("Unable to write to {}", config_file.display()))?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let conf = Self::soft_load()?;

        if !conf.ready() {
            bail!("configuration is not ready");
        }

        Ok(conf)
    }
}

pub fn command_config(args: &ConfigArgs) -> Result<()> {
    RMConfig::save(args)?;

    let conf = RMConfig::soft_load()?;

    let mut table = Table::new(vec![conf]);
    table.with(Style::modern_rounded());
    table.with(Rotate::Left);

    println!("{table}");

    Ok(())
}
