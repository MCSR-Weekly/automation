use crate::{RichPost, SecretString, get_http_client};
use anyhow::Result;
use atrium_xrpc_client::reqwest::{ReqwestClient, ReqwestClientBuilder};
use bsky_sdk::BskyAgent;
use bsky_sdk::agent::BskyAtpAgentBuilder;
use bsky_sdk::agent::config::Config;
use bsky_sdk::api::types::string::{Datetime, Language};
use log::info;
use serde::Deserialize;
use std::str::FromStr;

/// Bluesky client
///
/// # Credentials
/// - `MWA_BSKY_ENDPOINT`
/// - `MWA_BSKY_USERNAME`
/// - `MWA_BSKY_PASSWORD`
pub struct BskyClient {
    agent: BskyAgent<ReqwestClient>,
}

impl BskyClient {
    pub async fn new() -> Result<Self> {
        info!("logging in...");

        // -- load credentials --

        #[derive(Deserialize)]
        struct Creds {
            endpoint: String,
            username: String,
            password: SecretString,
        }
        let creds = envy::prefixed("MWA_BSKY_").from_env::<Creds>()?;

        // -- create client --

        let config = Config {
            endpoint: creds.endpoint,
            session: None,
            labelers_header: None,
            proxy_header: None,
        };

        let reqwest_client = ReqwestClientBuilder::new(&config.endpoint)
            .client(get_http_client())
            .build();

        let agent = BskyAtpAgentBuilder::new(reqwest_client)
            .config(config)
            .build()
            .await?;

        // -- check login status --

        let me = agent
            .login(&creds.username, creds.password.expose_secret())
            .await?;
        info!("authenticated as `{}`", me.handle.as_str());

        // --

        Ok(BskyClient { agent })
    }

    pub async fn post(&self, post: RichPost) -> Result<()> {
        info!("posting to bluesky");

        // -- build post --

        // least deranged bsky codebase
        let embed = bsky_sdk::api::app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedExternalMain(
            Box::new(
                bsky_sdk::api::app::bsky::embed::external::MainData {
                    external: bsky_sdk::api::app::bsky::embed::external::ExternalData {
                        title: post.embed_title,
                        description: post.embed_desc,
                        uri: post.url,
                        thumb: None,
                    }
                    .into(),
                }
                .into(),
            ),
        );

        // -- submit to api --

        self.agent
            .create_record(bsky_sdk::api::app::bsky::feed::post::RecordData {
                created_at: Datetime::now(),
                embed: Some(bsky_sdk::api::types::Union::Refs(embed)),
                entities: None,
                facets: None,
                labels: None,
                langs: Some(vec![Language::from_str("en").unwrap()]),
                reply: None,
                tags: None,
                text: post.message,
            })
            .await?;

        // --

        Ok(())
    }
}
