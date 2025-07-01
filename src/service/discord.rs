use crate::{ServiceClient, TextPost, get_http_client};
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
    config: Config,
}

#[derive(Deserialize)]
#[expect(unnameable_types)]
pub struct Creds {
    token: Token,
    #[serde(flatten)]
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    announcements_channel: GenericChannelId,
}

impl ServiceClient for DiscordClient {
    const NAME: &'static str = "discord";
    type Creds = Creds;

    async fn _create(creds: Creds) -> Result<Self> {
        let http = HttpBuilder::new(creds.token)
            .client(get_http_client())
            .default_allowed_mentions(CreateAllowedMentions::new())
            .build();

        Ok(DiscordClient {
            http,
            config: creds.config,
        })
    }

    async fn _login(&mut self) -> Result<()> {
        let me = self.http.get_current_user().await?;
        info!("authenticated as bot `{}` ({})", me.tag(), me.id);

        Ok(())
    }

    type CreatePostInput = TextPost;
    async fn create_post(&self, post: TextPost) -> Result<()> {
        let announcements_channel = self.config.announcements_channel;

        let message_builder = CreateMessage::new().content(post.content);
        let discord_message = announcements_channel
            .send_message(&self.http, message_builder)
            .await?;
        discord_message.crosspost(&self.http).await?;

        Ok(())
    }
}
