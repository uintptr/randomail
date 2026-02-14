use std::fmt::Display;

use anyhow::{Context, Result, bail};
use serde::Serialize;

pub const CF_API_URL: &str = "https://api.cloudflare.com/client/v4";
const CF_USER_AGENT: &str = "CFRelay 1.0";

pub fn issue_put<D, U, T>(url: U, api_token: T, data: &D) -> Result<()>
where
    T: AsRef<str>,
    U: AsRef<str> + Display,
    D: Serialize,
{
    let bearer = format!("Bearer {}", api_token.as_ref());

    let res = minreq::put(url.as_ref())
        .with_header("Authorization", bearer)
        .with_header("User-Agent", CF_USER_AGENT)
        .with_header("Content-Type", "application/json")
        .with_json(data)
        .context("Unable to serialize input data")?
        .send()
        .with_context(|| format!("Unable to issue DELETE to {}", url.as_ref()))?;

    if !(200..300).contains(&res.status_code) {
        if let Ok(data) = res.as_str() {
            bail!("{} returned {} {data}", url.as_ref(), res.status_code);
        } else {
            bail!("{} returned {}", url.as_ref(), res.status_code);
        }
    }

    Ok(())
}

pub fn issue_delete<U, T>(url: U, api_token: T) -> Result<()>
where
    T: AsRef<str>,
    U: AsRef<str> + Display,
{
    let bearer = format!("Bearer {}", api_token.as_ref());

    let res = minreq::delete(url.as_ref())
        .with_header("Authorization", bearer)
        .with_header("User-Agent", CF_USER_AGENT)
        .with_header("Content-Type", "application/json")
        .send()
        .with_context(|| format!("Unable to issue DELETE to {}", url.as_ref()))?;

    if !(200..300).contains(&res.status_code) {
        if let Ok(data) = res.as_str() {
            bail!("{} returned {} {data}", url.as_ref(), res.status_code);
        } else {
            bail!("{} returned {}", url.as_ref(), res.status_code);
        }
    }

    Ok(())
}

pub fn issue_post<D, U, T>(url: U, api_token: T, data: &D) -> Result<()>
where
    T: AsRef<str>,
    U: AsRef<str> + Display,
    D: Serialize,
{
    let bearer = format!("Bearer {}", api_token.as_ref());

    let res = minreq::post(url.as_ref())
        .with_header("Authorization", bearer)
        .with_header("User-Agent", CF_USER_AGENT)
        .with_header("Content-Type", "application/json")
        .with_json(data)
        .context("Unable to serialize input data")?
        .send()
        .with_context(|| format!("Unable to issue POST to {}", url.as_ref()))?;

    if !(200..300).contains(&res.status_code) {
        if let Ok(data) = res.as_str() {
            bail!("{} returned {} {data}", url.as_ref(), res.status_code);
        } else {
            bail!("{} returned {}", url.as_ref(), res.status_code);
        }
    }

    Ok(())
}

pub fn issue_get<U, T>(url: U, api_token: T) -> Result<String>
where
    T: AsRef<str> + Display,
    U: AsRef<str>,
{
    let bearer = format!("Bearer {api_token}");

    let res = minreq::get(url.as_ref())
        .with_header("Authorization", bearer)
        .with_header("User-Agent", CF_USER_AGENT)
        .send()
        .with_context(|| format!("Unable to issue GET {}", url.as_ref()))?;

    if res.status_code < 200 || res.status_code >= 300 {
        if let Ok(data) = res.as_str() {
            bail!("{} returned {} {data}", url.as_ref(), res.status_code);
        } else {
            bail!("{} returned {}", url.as_ref(), res.status_code);
        }
    }

    let data = res
        .as_str()
        .context("Unable to read string from response")?;

    Ok(data.into())
}
