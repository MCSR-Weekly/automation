#[macro_use] // avoid importing logging macros everywhere
extern crate log;

mod credentials;
mod http;
mod secret;
mod service;

use anyhow::Result;
use service::*;

pub use self::secret::SecretString;

#[derive(Debug, Clone)]
pub struct PostContent {
    pub message: String,
    pub embed_title: String,
    pub embed_desc: String,
    pub url: String,
    pub raw_text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    info!("MCSR-Weekly-Automation v{}", env!("CARGO_PKG_VERSION"));

    let content = PostContent {
        message: "MCSR Weekly Issue #4 is now available!".to_string(),
        embed_title: "Title".to_string(),
        embed_desc: "description".to_string(),
        url: "https://mcsrweekly.net/posts/test".to_string(),
        raw_text: "MCSR Weekly issue #4 is now available!\nhttps://mcsrweekly.net/posts/test"
            .to_string(),
    };
    info!("Content to post: {content:#?}");

    credentials::load_from_file().await?;

    bluesky::post(content.clone()).await?;
    discord::post(content.clone()).await?;
    mastodon::post(content.clone()).await?;

    Ok(())
}

fn init_logging() {
    use log::LevelFilter;

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .filter_module(module_path!(), LevelFilter::Trace)
        .parse_default_env()
        .format_timestamp(None)
        .init();
}
