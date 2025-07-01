use crate::{PostContent, SecretString, credentials, http};
use anyhow::Result;
use atrium_xrpc_client::reqwest::ReqwestClientBuilder;
use bsky_sdk::BskyAgent;
use bsky_sdk::agent::BskyAtpAgentBuilder;
use bsky_sdk::agent::config::Config;
use bsky_sdk::api::types::string::{Datetime, Language};
use serde::Deserialize;
use std::str::FromStr;
use tokio::sync::OnceCell;

#[derive(Debug, Deserialize)]
pub struct BlueskyCredentials {
    endpoint: String,
    username: String,
    password: SecretString,
}

// least deranged bsky codebase
pub async fn post(post: PostContent) -> Result<()> {
    info!("posting to bluesky");

    let agent = get_agent().await?;

    let embed =
        bsky_sdk::api::app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
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
        ));
    agent
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

// --

async fn get_agent() -> Result<BskyAgent> {
    async fn build_agent() -> Result<BskyAgent> {
        let creds = &credentials::get().bluesky;

        let config = Config {
            endpoint: creds.endpoint.clone(),
            session: None,
            labelers_header: None,
            proxy_header: None,
        };

        let reqwest_client = ReqwestClientBuilder::new(&config.endpoint)
            .client(http::get_client())
            .build();

        let agent = BskyAtpAgentBuilder::new(reqwest_client)
            .config(config)
            .build()
            .await?;

        agent
            .login(&creds.username, creds.password.expose_secret())
            .await?;

        Ok(agent)
    }

    static AGENT: OnceCell<BskyAgent> = OnceCell::const_new();
    Ok(AGENT.get_or_try_init(build_agent).await?.clone())
}
