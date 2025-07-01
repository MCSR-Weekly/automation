use crate::{RichPost, SecretString, ServiceClient, get_http_client};
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
    creds: Creds,
}

#[derive(Deserialize)]
#[expect(unnameable_types)]
pub struct Creds {
    endpoint: String,
    username: String,
    password: SecretString,
}

impl ServiceClient for BskyClient {
    const NAME: &'static str = "bsky";
    type Creds = Creds;

    async fn _create(creds: Creds) -> Result<Self> {
        let config = Config {
            endpoint: creds.endpoint.clone(),
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

        Ok(BskyClient { agent, creds })
    }

    async fn _login(&mut self) -> Result<()> {
        let me = self
            .agent
            .login(&self.creds.username, self.creds.password.expose_secret())
            .await?;
        info!("authenticated as `{}`", me.handle.as_str());

        Ok(())
    }

    // least deranged bsky codebase
    type CreatePostInput = RichPost;
    async fn create_post(&self, post: RichPost) -> Result<()> {
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

        Ok(())
    }
}
