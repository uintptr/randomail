use anyhow::{Context, Result, bail};

pub const CF_API_URL: &str = "https://api.cloudflare.com/client/v4";
const CF_USER_AGENT: &str = "CFRelay 1.0";

pub fn issue_get(url: &str, api_token: &str) -> Result<String> {
    let bearer = format!("Bearer {api_token}");

    let res = minreq::get(url)
        .with_header("Authorization", bearer)
        .with_header("User-Agent", CF_USER_AGENT)
        .send()
        .with_context(|| format!("Unable to issue GET {}", url))?;

    if res.status_code < 200 || res.status_code >= 300 {
        if let Ok(data) = res.as_str() {
            bail!("{} returned {} {data}", url, res.status_code);
        } else {
            bail!("{} returned {}", url, res.status_code);
        }
    }

    let data = res
        .as_str()
        .context("Unable to read string from response")?;

    Ok(data.into())
}
