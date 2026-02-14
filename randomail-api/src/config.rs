use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use log::info;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{PROJECT_NAME, cf_destination::destination_address, cf_zone::zone_info};

const CONFIG_FILE_NAME: &str = "config.json";

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
    pub fn soft_load_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut fd = fs::OpenOptions::new()
            .read(true)
            .open(&path)
            .with_context(|| format!("Unable to open {} for reading", path.as_ref().display()))?;

        let mut data = String::new();

        info!("reading {}", path.as_ref().display());

        fd.read_to_string(&mut data)
            .with_context(|| format!("Unable to read {}", path.as_ref().display()))?;

        let token: Self = serde_json::from_str(&data)
            .with_context(|| format!("Unable to deserialize {}", path.as_ref().display()))?;

        Ok(token)
    }

    pub fn soft_load() -> Result<Self> {
        let config_file = get_config_file()?;
        Self::soft_load_path(config_file)
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

    pub fn update(
        &mut self,
        account_id: Option<String>,
        token: Option<String>,
        email: Option<String>,
        domain: Option<String>,
    ) -> Result<()> {
        if let Some(account_id) = account_id {
            self.account_id = account_id;
        }

        if let Some(token) = token {
            self.token = token;
        }

        if let Some(email) = email {
            if self.token.is_empty() {
                bail!("token is missing")
            }

            if self.account_id.is_empty() {
                bail!("account_id is missing")
            }

            let dst = destination_address(&self.account_id, &email, &self.token)
                .with_context(|| format!("Unable to get email id for  {email}"))?;

            self.destination_email = email;
            self.destination_email_id = dst.id;
        }

        if let Some(zone) = domain {
            if self.token.is_empty() {
                bail!("token is missing")
            }

            let zinfo = zone_info(&zone, &self.token)
                .with_context(|| format!("Unable to get zone info for {zone}"))?;

            self.zone = zone;
            self.zone_id = zinfo.id;
        }

        self.save()
    }

    fn save(&self) -> Result<()> {
        let config_file = get_config_file()?;

        let encoded_data =
            serde_json::to_string_pretty(self).context("Unable to serialize data")?;

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
        let config_file = get_config_file()?;
        Self::load_from_path(config_file)
    }

    pub fn load_from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let conf = Self::soft_load_path(path)?;

        if !conf.ready() {
            bail!("configuration is not ready");
        }

        Ok(conf)
    }
}
