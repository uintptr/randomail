use std::fmt::Display;

use anyhow::Result;
use tabled::{Table, settings::Style};

use crate::{
    cf_email::{add_email_route, list_email_routes},
    config::RMConfig,
};

pub fn command_add<A, D>(alias: A, description: D) -> Result<()>
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

pub fn command_list() -> Result<()> {
    let conf = RMConfig::load()?;

    let routes = list_email_routes(&conf.zone_id, &conf.token)?;

    let mut table = Table::new(&routes);
    table.with(Style::modern_rounded());

    println!("{table}");

    Ok(())
}
