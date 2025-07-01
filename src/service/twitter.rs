use crate::{PostContent, SecretString, credentials};
use anyhow::Result;
use serde::Deserialize;
use tokio::sync::OnceCell;
use tweety_rs::TweetyClient;
use tweety_rs::api::user::{UserFields, UserQueryParams};

#[derive(Debug, Deserialize)]
pub struct TwitterCredentials {
    api_key: SecretString,
    api_secret: SecretString,
    access_token: SecretString,
    access_secret: SecretString,
}

// --

pub async fn post(post: PostContent) -> Result<()> {
    info!("posting to twitter");

    let client = get_client().await?;

    client.post_tweet(&post.raw_text, None).await?;

    Ok(())
}

// --

async fn get_client() -> Result<&'static TweetyClient> {
    async fn build_client() -> Result<TweetyClient> {
        let creds = &credentials::get().twitter;

        let client = TweetyClient::new(
            creds.api_key.expose_secret(),
            creds.access_token.expose_secret(),
            creds.api_secret.expose_secret(),
            creds.access_secret.expose_secret(),
        );

        let me = client
            .get_user_me(Some(UserQueryParams {
                expansions: None,
                tweet_fields: None,
                user_fields: Some(vec![UserFields::Username]),
            }))
            .await?
            .response;
        let me = GetAuthenticatedUser::deserialize(me)?;
        info!("authenticated as @{}", me.username);

        Ok(client)
    }

    static CLIENT: OnceCell<TweetyClient> = OnceCell::const_new();
    CLIENT.get_or_try_init(build_client).await
}

#[derive(Debug, Deserialize)]
struct GetAuthenticatedUser {
    username: String,
}
