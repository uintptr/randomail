use std::fmt::Display;

use anyhow::{Context, Result, bail};
use serde::Serialize;

pub const CF_API_URL: &str = "https://api.cloudflare.com/client/v4";
const CF_USER_AGENT: &str = "RandoMail 1.0";

pub async fn issue_put<D, U, T>(url: U, api_token: T, data: &D) -> Result<()>
where
    T: AsRef<str> + Display,
    U: AsRef<str> + Display,
    D: Serialize,
{
    let client = reqwest::Client::new();

    let res = client
        .put(url.as_ref())
        .bearer_auth(api_token)
        .header("User-Agent", CF_USER_AGENT)
        .header("Content-Type", "application/json")
        .json(data)
        .send()
        .await
        .with_context(|| format!("Unable to issue PUT to {}", url.as_ref()))?;

    let status = res.status();

    if !res.status().is_success() {
        if let Ok(data) = res.text().await {
            bail!("{} returned {} {data}", url.as_ref(), status);
        }
        bail!("{} returned {}", url.as_ref(), status);
    }

    Ok(())
}

pub async fn issue_delete<U, T>(url: U, api_token: T) -> Result<()>
where
    T: AsRef<str> + Display,
    U: AsRef<str> + Display,
{
    let client = reqwest::Client::new();

    let res = client
        .delete(url.as_ref())
        .bearer_auth(api_token)
        .header("User-Agent", CF_USER_AGENT)
        .header("Content-Type", "application/json")
        .send()
        .await
        .with_context(|| format!("Unable to issue DELETE to {}", url.as_ref()))?;

    let status = res.status();

    if !res.status().is_success() {
        if let Ok(data) = res.text().await {
            bail!("{} returned {} {data}", url.as_ref(), status);
        }
        bail!("{} returned {}", url.as_ref(), status);
    }

    Ok(())
}

pub async fn issue_post<D, U, T>(url: U, api_token: T, data: &D) -> Result<()>
where
    T: AsRef<str> + Display,
    U: AsRef<str> + Display,
    D: Serialize,
{
    let client = reqwest::Client::new();

    let res = client
        .post(url.as_ref())
        .bearer_auth(api_token)
        .header("User-Agent", CF_USER_AGENT)
        .header("Content-Type", "application/json")
        .json(data)
        .send()
        .await
        .with_context(|| format!("Unable to issue POST to {}", url.as_ref()))?;

    let status = res.status();

    if !res.status().is_success() {
        if let Ok(data) = res.text().await {
            bail!("{} returned {} {data}", url.as_ref(), status);
        }
        bail!("{} returned {}", url.as_ref(), status);
    }

    Ok(())
}

pub async fn issue_get<U, T>(url: U, api_token: T) -> Result<String>
where
    T: AsRef<str> + Display,
    U: AsRef<str> + Display,
{
    let client = reqwest::Client::new();

    let res = client
        .get(url.as_ref())
        .bearer_auth(api_token)
        .header("User-Agent", CF_USER_AGENT)
        .send()
        .await
        .with_context(|| format!("Unable to issue GET {}", url.as_ref()))?;

    let status = res.status();
    let data = res
        .text()
        .await
        .with_context(|| format!("Unable to GET data from{}", url.as_ref()))?;

    if !status.is_success() {
        bail!("{url} returned {status} {data}");
    }

    Ok(data)
}
