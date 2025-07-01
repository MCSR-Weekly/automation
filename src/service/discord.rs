use crate::{PostContent, credentials, http};
use anyhow::Result;
use serde::Deserialize;
use serenity::builder::{CreateAllowedMentions, CreateMessage};
use serenity::http::{Http, HttpBuilder};
use serenity::model::id::GenericChannelId;
use serenity::secrets::Token;
use tokio::sync::OnceCell;

#[derive(Debug, Deserialize)]
pub struct DiscordCredentials {
    bot_token: Token,
    announcements_channel: GenericChannelId,
}

// --

pub async fn post(post: PostContent) -> Result<()> {
    info!("posting to discord");

    let http = get_client().await?;
    let announcements_channel = credentials::get().discord.announcements_channel;

    // let embed = CreateEmbed::new()
    //     .author(CreateEmbedAuthor::new("MCSR Weekly"))
    //     .title(post.embed_title)
    //     .description(post.embed_desc)
    //     .url(post.url)
    //     .timestamp(Timestamp::now());
    // let message_builder = CreateMessage::new().content(post.message).embed(embed);
    let message_builder = CreateMessage::new().content(post.message);

    let discord_message = announcements_channel
        .send_message(http, message_builder)
        .await?;
    discord_message.crosspost(http).await?;

    Ok(())
}

// --

async fn get_client() -> Result<&'static Http> {
    async fn build_client() -> Result<Http> {
        let client = HttpBuilder::new(credentials::get().discord.bot_token.clone())
            .client(http::get_client())
            .default_allowed_mentions(CreateAllowedMentions::new())
            .build();

        let bot_user = client.get_current_user().await?;
        info!(
            "authenticated as bot `{}` ({})",
            bot_user.tag(),
            bot_user.id
        );

        Ok(client)
    }

    static CLIENT: OnceCell<Http> = OnceCell::const_new();
    CLIENT.get_or_try_init(build_client).await
}
