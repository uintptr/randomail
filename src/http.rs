use anyhow::{Context, Result, bail};

pub const CF_API_URL: &str = "https://api.cloudflare.com/client/v4";
const CF_USER_AGENT: &str = "CFRelay 1.0";

pub fn issue_get<U, T>(url: U, api_token: T) -> Result<String>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let bearer = format!("Bearer {}", api_token.as_ref());

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
