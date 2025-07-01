use crate::{TextPost, get_http_client};
use anyhow::Result;
use log::info;
use serde::Deserialize;
use serenity::builder::{CreateAllowedMentions, CreateMessage};
use serenity::http::{Http, HttpBuilder};
use serenity::model::id::GenericChannelId;
use serenity::secrets::Token;

/// Discord client
///
/// # Credentials
/// - `MWA_DISCORD_TOKEN`
///
/// # Config
/// - `MWA_DISCORD_ANNOUCEMENTS_CHANNEL` - [DiscordClient::post]
pub struct DiscordClient {
    // access to discord's REST api, via serenity
    http: Http,
    config: DiscordConfig,
}

#[derive(Deserialize)]
struct DiscordConfig {
    announcements_channel: GenericChannelId,
}

impl DiscordClient {
    pub async fn new() -> Result<Self> {
        info!("logging in...");

        // -- load credentials --

        #[derive(Deserialize)]
        struct Creds {
            token: Token,
            #[serde(flatten)]
            config: DiscordConfig,
        }
        let creds = envy::prefixed("MWA_DISCORD_").from_env::<Creds>()?;

        // -- create client --

        let http = HttpBuilder::new(creds.token)
            .client(get_http_client())
            .default_allowed_mentions(CreateAllowedMentions::new())
            .build();

        // -- check login status --

        let me = http.get_current_user().await?;
        info!("authenticated as bot `{}` ({})", me.tag(), me.id);

        // --

        Ok(DiscordClient {
            http,
            config: creds.config,
        })
    }

    pub async fn post(&self, post: TextPost) -> Result<()> {
        info!("posting to discord");

        // -- load config --

        let announcements_channel = self.config.announcements_channel;

        // -- build post --

        let message_builder = CreateMessage::new().content(post.content);

        // -- submit to api --

        let discord_message = announcements_channel
            .send_message(&self.http, message_builder)
            .await?;
        discord_message.crosspost(&self.http).await?;

        // --

        Ok(())
    }
}
