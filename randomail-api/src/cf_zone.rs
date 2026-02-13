use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::http::{CF_API_URL, issue_get};

#[derive(Deserialize)]
pub struct CFZoneInfo {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
struct CFZoneInfoResponse {
    result: Vec<CFZoneInfo>,
}

pub fn zone_info<D, T>(domain: D, token: T) -> Result<CFZoneInfo>
where
    D: AsRef<str>,
    T: AsRef<str>,
{
    let url = format!("{CF_API_URL}/zones");

    let data = issue_get(url, token)?;

    let response: CFZoneInfoResponse =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    for r in response.result {
        if r.name == domain.as_ref() {
            return Ok(r);
        }
    }

    bail!("{} was not found in response", domain.as_ref())
}
