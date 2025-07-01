use anyhow::Result;
use log::info;
use mwa::{BskyClient, DiscordClient, MastodonClient, ServiceClient, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    _ = dotenvy::dotenv();
    init_logging();
    info!(
        "MCSR-Weekly-Automation v{} - {}",
        env!("CARGO_PKG_VERSION"),
        module_path!(),
    );

    _ = BskyClient::new().await?;
    _ = DiscordClient::new().await?;
    _ = MastodonClient::new().await?;
    _ = TwitterClient::new().await?;

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
