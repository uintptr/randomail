use anyhow::{Context, Result, bail};

use crate::{
    config::MailToken,
    http::{CF_API_URL, issue_get},
    responses::{CFEmailDestination, CFEmailDestinations},
};

fn get_dests() -> Result<CFEmailDestinations> {
    let mail_token = MailToken::load()?;

    let url = format!(
        "{CF_API_URL}/accounts/{}/email/routing/addresses",
        mail_token.account_id
    );

    let data = issue_get(&url, &mail_token.token)?;

    let dests: CFEmailDestinations =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    if !dests.success {
        if let Some(msg) = dests.messages.first() {
            bail!("{msg}")
        }
        bail!("{url} failed");
    }

    Ok(dests)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC
////////////////////////////////////////////////////////////////////////////////

pub fn default_destinaton() -> Result<CFEmailDestination> {
    let dests = get_dests()?;

    if let Some(default) = dests.result.into_iter().next() {
        return Ok(default);
    }

    bail!("No default destination found")
}

pub fn command_destinatons() -> Result<()> {
    let dests = get_dests()?;

    println!("Destination Emails:");
    for e in dests.result {
        println!("    {} ({})", e.email, e.id);
    }

    Ok(())
}
