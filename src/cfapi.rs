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

#[derive(Deserialize)]
pub struct CFDestinationAddr {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize)]
struct CFDestinationAddrsResponse {
    result: Vec<CFDestinationAddr>,
}

#[derive(Debug, Deserialize)]
pub struct CFEmailRouteMatch {
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CFEmailRouteAction {
    value: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CFEmailRoute {
    pub id: String,
    pub name: String,
    pub actions: Vec<CFEmailRouteAction>,
    pub matchers: Vec<CFEmailRouteMatch>,
}

#[derive(Deserialize)]
struct CFEmailRoutesResponse {
    result: Vec<CFEmailRoute>,
}

pub fn destination_address<A, E, T>(account_id: A, email: E, token: T) -> Result<CFDestinationAddr>
where
    A: AsRef<str>,
    E: AsRef<str>,
    T: AsRef<str>,
{
    let url = format!(
        "{CF_API_URL}/accounts/{}/email/routing/addresses",
        account_id.as_ref()
    );

    let data = issue_get(url, token)?;

    let response: CFDestinationAddrsResponse =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    for r in response.result {
        if r.email == email.as_ref() {
            return Ok(r);
        }
    }

    bail!("{} was not found in response", email.as_ref())
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

pub fn list_email_forward_route<Z, E, T>(zone_id: Z, email: E, token: T) -> Result<CFEmailRoute>
where
    Z: AsRef<str>,
    E: AsRef<str>,
    T: AsRef<str>,
{
    let url = format!(
        "{CF_API_URL}//zones/{}/email/routing/rules",
        zone_id.as_ref()
    );

    let data = issue_get(url, token)?;

    let response: CFEmailRoutesResponse =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    for r in response.result {
        //
        // see if the email is in the actions
        //
        let mut email_match = false;

        for a in r.actions.iter() {
            if let Some(values) = &a.value {
                if values.iter().any(|v| v == email.as_ref()) {
                    email_match = true;
                    break;
                }
            }
        }

        if !email_match {
            continue;
        }

        return Ok(r);
    }

    bail!("{} was not found in response", email.as_ref())
}
