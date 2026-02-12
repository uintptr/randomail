use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::common::PROJECT_NAME;

#[derive(Args)]
pub struct ConfigArgs {
    /// Account ID
    #[arg(long, short = 'i')]
    account_id: Option<String>,

    /// API Token
    #[arg(long, short)]
    token: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CFConfig {
    pub account_id: String,
    pub token: String,
}

fn get_config_file() -> Result<PathBuf> {
    let config_root = dirs::config_dir().context("Unable to find config directory")?;

    let config_dir = config_root.join(PROJECT_NAME);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Unable to create {}", config_dir.display()))?;
    }

    Ok(config_dir.join("token.json"))
}

impl CFConfig {
    /*
    pub fn save(args: &CFConfigArgs) -> Result<()> {
        let config_file = get_config_file()?;

        let data = Self {
            account_id: account_id.into(),
            token: token.into(),
        };

        let encoded_data =
            serde_json::to_string_pretty(&data).context("Unable to serialize data")?;

        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_file)
            .with_context(|| format!("Unable to open {} for writing", config_file.display()))?;

        fd.write_all(encoded_data.as_bytes())
            .with_context(|| format!("Unable to write to {}", config_file.display()))?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let config_file = get_config_file()?;

        let mut fd = fs::OpenOptions::new()
            .read(true)
            .open(&config_file)
            .with_context(|| format!("Unable to open {} for reading", config_file.display()))?;

        let mut data = String::new();

        fd.read_to_string(&mut data)
            .with_context(|| format!("Unable to read {}", config_file.display()))?;

        let token: Self = serde_json::from_str(&data)
            .with_context(|| format!("Unable to deserialize {}", config_file.display()))?;

        Ok(token)
    }
    */
}

pub fn command_config(_args: &ConfigArgs) -> Result<()> {
    Ok(())
}
