//! mcsr weekly automation tooling

#[macro_use]
mod macros;

mod cli;
mod http;
mod post;
mod secret;
mod service;

use crate::cli::Cli;
use crate::http::get_http_client;
use crate::post::{Post, RichPost, TextPost};
use crate::secret::SecretString;
use crate::service::{BskyClient, DiscordClient, MastodonClient, ServiceClient, TwitterClient};
use anyhow::Result;
use clap::Parser;
use std::process::ExitCode;

// --

fn main() -> ExitCode {
    let cli = Cli::parse();

    init_logging();
    init_dotenv();
    info!("MCSR-Weekly-Automation v{}", env!("CARGO_PKG_VERSION"));

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("FATAL: {err}");
            for e in err.chain().skip(1) {
                error!("Cause: {e}");
            }

            ExitCode::FAILURE
        }
    }
}

#[tokio::main]
async fn run(cli: Cli) -> Result<()> {
    match cli {
        Cli::CheckCreds => {
            _ = BskyClient::new().await?;
            _ = DiscordClient::new().await?;
            _ = MastodonClient::new().await?;
            _ = TwitterClient::new().await?;
        }
    }

    Ok(())
}

// -- init --

fn init_logging() {
    use log::{LevelFilter, Log, Metadata, Record};

    struct Adapter<L>(L);
    impl<L: Log> Log for Adapter<L> {
        fn enabled(&self, metadata: &Metadata) -> bool {
            self.0.enabled(metadata)
        }

        fn log(&self, record: &Record) {
            let mut rb = record.to_builder();

            // clear the target if it is the same as the module path, to prevent double printing it
            if Some(record.target()) == record.module_path() {
                rb.target("");
            }

            let r = rb.build();
            self.0.log(&r);
        }

        fn flush(&self) {
            self.0.flush();
        }
    }

    let logger = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .filter_module("mwa", LevelFilter::Trace)
        .format_module_path(true)
        .format_timestamp(None)
        .build();

    let filter = logger.filter();
    log::set_boxed_logger(Box::new(Adapter(logger))).unwrap();
    log::set_max_level(filter);
}

fn init_dotenv() {
    match dotenvy::dotenv() {
        Ok(path) => debug!(dotenv: "loaded `{}`", path.display()),
        Err(err) => warn!(dotenv: "{err}"),
    }
}

/*
use anyhow::Result;
use log::info;
use mwa::{
    BskyClient, DiscordClient, MastodonClient, Post, RichPost, ServiceClient, TextPost,
    TwitterClient,
};

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
    bsky.create_post(post.rich.clone()).await?;

    let discord = DiscordClient::new().await?;
    discord.create_post(post.text.clone()).await?;

    let mastodon = MastodonClient::new().await?;
    mastodon.create_post(post.text.clone()).await?;

    let twitter = TwitterClient::new().await?;
    twitter.create_post(post.text.clone()).await?;

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
*/
