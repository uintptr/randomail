use std::fmt::Display;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::http::{CF_API_URL, issue_get};

#[derive(Deserialize)]
pub struct CFDestinationAddr {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize)]
struct CFDestinationAddrsResponse {
    result: Vec<CFDestinationAddr>,
}

pub async fn destination_address<A, E, T>(
    account_id: A,
    email: E,
    token: T,
) -> Result<CFDestinationAddr>
where
    A: AsRef<str>,
    E: AsRef<str>,
    T: AsRef<str> + Display,
{
    let url = format!(
        "{CF_API_URL}/accounts/{}/email/routing/addresses",
        account_id.as_ref()
    );

    let data = issue_get(url, token).await?;

    let response: CFDestinationAddrsResponse =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    for r in response.result {
        if r.email == email.as_ref() {
            return Ok(r);
        }
    }

    bail!("{} was not found in response", email.as_ref())
}
