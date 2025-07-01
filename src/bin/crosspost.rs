use anyhow::Result;
use log::info;
use mwa::{BskyClient, DiscordClient, MastodonClient, Post, RichPost, TextPost, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    init_logging();
    info!(
        "MCSR-Weekly-Automation v{} - {}",
        env!("CARGO_PKG_VERSION"),
        module_path!(),
    );

    let post = Post {
        rich: RichPost {
            message: "MCSR Weekly Issue #2 is now available!".to_string(),
            embed_title: "Dummy Title".to_string(),
            embed_desc: "Dummy Description".to_string(),
            url: "https://mcsrweekly.net/posts/test".to_string(),
        },
        text: TextPost {
            content: "MCSR Weekly issue #2 is now available!\nhttps://mcsrweekly.net/issue/2"
                .to_string(),
        },
    };
    info!("Content to post: {post:#?}");

    let bsky = BskyClient::new().await?;
    bsky.post(post.rich.clone()).await?;

    let discord = DiscordClient::new().await?;
    discord.post(post.text.clone()).await?;

    let mastodon = MastodonClient::new().await?;
    mastodon.post(post.text.clone()).await?;

    let twitter = TwitterClient::new().await?;
    twitter.post(post.text.clone()).await?;

    Ok(())
}

fn init_logging() {
    use log::LevelFilter;

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .filter_module("mwa", LevelFilter::Trace)
        .filter_module(module_path!(), LevelFilter::Trace)
        .parse_default_env()
        .format_timestamp(None)
        .init();
}
