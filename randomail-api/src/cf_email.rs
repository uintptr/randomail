use std::fmt::Display;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::http::{CF_API_URL, issue_delete, issue_get, issue_post, issue_put};

#[derive(Debug, Serialize, Deserialize)]
struct CFEmailRouteMatch {
    #[serde(rename = "type")]
    action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CFEmailRouteMatch {
    pub fn new<E>(email: E) -> Self
    where
        E: Into<String>,
    {
        Self {
            action_type: "literal".to_string(),
            field: Some("to".to_string()),
            value: Some(email.into()),
        }
    }

    pub fn email_alias(&self) -> Result<String> {
        if let Some(value) = &self.value {
            return Ok(value.into());
        }

        bail!("email alias not found")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CFEmailRouteAction {
    #[serde(rename = "type")]
    action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<String>>,
}

impl CFEmailRouteAction {
    pub fn new<E>(email: E) -> Self
    where
        E: Into<String>,
    {
        Self {
            action_type: "forward".to_string(),
            value: Some(vec![email.into()]),
        }
    }

    pub fn email_dest(&self) -> Result<String> {
        if let Some(values) = &self.value
            && let Some(dst) = values.first()
        {
            return Ok(dst.clone());
        }

        bail!("email destination not found")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CFEmailRoute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub actions: Vec<CFEmailRouteAction>,
    pub matchers: Vec<CFEmailRouteMatch>,
    pub enabled: bool,
}

impl CFEmailRoute {
    pub fn new<N, D, A>(route_name: N, email_alias: A, email_dst: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        A: Into<String>,
    {
        let actions = vec![CFEmailRouteAction::new(email_dst)];

        let matchers = vec![CFEmailRouteMatch::new(email_alias)];

        Self {
            id: None,
            name: Some(route_name.into()),
            actions,
            matchers,
            enabled: true,
        }
    }

    pub fn email_alias(&self) -> Result<String> {
        let entry = self
            .matchers
            .first()
            .context("invalid context, matches missing")?;
        entry.email_alias()
    }

    pub fn email_dest(&self) -> Result<String> {
        let entry = self
            .actions
            .first()
            .context("invalid context, actions missing")?;
        entry.email_dest()
    }
}

#[derive(Serialize, Deserialize)]
struct CFEmailRouting {
    result: Vec<CFEmailRoute>,
}

#[derive(Debug, Default, Tabled, Serialize)]
pub struct RMAlias {
    #[tabled(skip)]
    pub id: String,
    #[tabled(skip)]
    pub email_destination: String,
    #[tabled(rename = "alias")]
    pub email_alias: String,
    pub name: String,
    pub enabled: bool,
}

impl TryFrom<CFEmailRoute> for RMAlias {
    type Error = anyhow::Error;

    fn try_from(route: CFEmailRoute) -> std::result::Result<Self, Self::Error> {
        let Some(id) = &route.id else {
            bail!("invalid context, id missing");
        };

        let email_destination = route.email_dest()?;
        let email_alias = route.email_alias()?;

        Ok(Self {
            id: id.into(),
            name: route.name.unwrap_or_default(),
            email_destination,
            email_alias,
            enabled: route.enabled,
        })
    }
}

async fn find_route<Z, E, T>(zone_id: Z, email_id: E, token: T) -> Result<CFEmailRoute>
where
    Z: AsRef<str> + Display,
    T: AsRef<str> + Display,
    E: AsRef<str> + Display,
{
    let url = format!("{CF_API_URL}/zones/{zone_id}/email/routing/rules",);

    let data = issue_get(url, token).await?;

    let response: CFEmailRouting =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    for r in response.result {
        if let Some(id) = &r.id
            && id.as_str() == email_id.as_ref()
        {
            return Ok(r);
        }
    }

    bail!("route not found for {email_id}")
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC
////////////////////////////////////////////////////////////////////////////////

pub async fn delete_email_route<Z, I, T>(zone_id: Z, email_id: I, token: T) -> Result<()>
where
    Z: AsRef<str> + Display,
    I: AsRef<str> + Display,
    T: AsRef<str>,
{
    let url = format!("{CF_API_URL}/zones/{zone_id}/email/routing/rules/{email_id}");

    issue_delete(url, token).await
}

pub async fn add_email_route<Z, N, A, D, T>(
    zone_id: Z,
    name: N,
    email_alias: A,
    email_dest: D,
    token: T,
) -> Result<()>
where
    Z: AsRef<str> + Display,
    N: Into<String> + Display,
    A: Into<String> + Display,
    D: Into<String> + Display,
    T: AsRef<str> + Display,
{
    let url = format!("{CF_API_URL}/zones/{zone_id}/email/routing/rules");

    let route = CFEmailRoute::new(name, email_alias, email_dest);

    issue_post(url, token, &route).await
}

pub async fn update_email_route<Z, I, T>(
    zone_id: Z,
    email_id: I,
    token: T,
    enabled: bool,
) -> Result<()>
where
    Z: AsRef<str> + Display,
    I: AsRef<str> + Display,
    T: AsRef<str> + Display,
{
    let url = format!("{CF_API_URL}/zones/{zone_id}/email/routing/rules/{email_id}");

    let mut route = find_route(zone_id, email_id, &token).await?;

    route.enabled = enabled;

    issue_put(url, token, &route).await
}

pub async fn rename_email_route<Z, I, T, N>(
    zone_id: Z,
    email_id: I,
    token: T,
    name: N,
) -> Result<()>
where
    Z: AsRef<str> + Display,
    I: AsRef<str> + Display,
    T: AsRef<str> + Display,
    N: Into<String> + Display,
{
    let url = format!("{CF_API_URL}/zones/{zone_id}/email/routing/rules/{email_id}");

    let mut route = find_route(zone_id, email_id, &token).await?;

    route.name = Some(name.into());

    issue_put(url, token, &route).await
}

pub async fn list_email_routes<Z, T>(zone_id: Z, token: T) -> Result<Vec<RMAlias>>
where
    Z: AsRef<str>,
    T: AsRef<str> + Display,
{
    let url = format!(
        "{CF_API_URL}/zones/{}/email/routing/rules",
        zone_id.as_ref()
    );

    let data = issue_get(url, token).await?;

    let response: CFEmailRouting =
        serde_json::from_str(&data).with_context(|| format!("Unable to deserialize {data}"))?;

    let mut aliases = Vec::new();

    for r in response.result {
        if let Ok(alias) = TryInto::<RMAlias>::try_into(r) {
            aliases.push(alias);
        }
    }

    Ok(aliases)
}
