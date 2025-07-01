use crate::service::{
    bluesky::BlueskyCredentials, discord::DiscordCredentials, mastodon::MastodonCredentials,
};
use anyhow::Result;
use serde::Deserialize;
use std::sync::OnceLock;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub bluesky: BlueskyCredentials,
    pub discord: DiscordCredentials,
    pub mastodon: MastodonCredentials,
    // pub twitter: TwitterCredentials,
}

static CREDENTIALS: OnceLock<Credentials> = OnceLock::new();

pub fn get() -> &'static Credentials {
    CREDENTIALS.get().unwrap()
}

pub async fn load_from_file() -> Result<()> {
    let creds_file = fs::read_to_string("credentials.toml").await?;
    let creds = toml::from_str(&creds_file)?;
    debug!("Loaded credentials: {creds:#?}");
    assert!(CREDENTIALS.set(creds).is_ok());

    Ok(())
}
