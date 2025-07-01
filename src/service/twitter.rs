use crate::{SecretString, TextPost};
use anyhow::Result;
use log::{info, trace};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use tweety_rs::TweetyClient;
use tweety_rs::api::error::TweetyError;
use tweety_rs::types::types::ResponseWithHeaders;

/// Twitter client
///
/// # Credentials
/// - `MWA_TWITTER_API_KEY`
/// - `MWA_TWITTER_API_SECRET`
/// - `MWA_TWITTER_ACCESS_TOKEN`
/// - `MWA_TWITTER_ACCESS_SECRET`
pub struct TwitterClient {
    client: TweetyClient,
}

impl TwitterClient {
    pub async fn new() -> Result<Self> {
        info!("logging in...");

        // -- load credentials --

        #[derive(Deserialize)]
        struct Creds {
            api_key: SecretString,
            api_secret: SecretString,
            access_token: SecretString,
            access_secret: SecretString,
        }
        let creds = envy::prefixed("MWA_TWITTER_").from_env::<Creds>()?;

        // -- create client --

        let client = TweetyClient::new(
            creds.api_key.expose_secret(),
            creds.access_token.expose_secret(),
            creds.api_secret.expose_secret(),
            creds.access_secret.expose_secret(),
        );
        let client = TwitterClient { client };

        // -- check login status --

        #[derive(Deserialize)]
        struct CurrentUser {
            // id: String,
            // name: String,
            username: String,
        }
        let me: ApiResponse<CurrentUser> = client.req(async |c| c.get_user_me(None).await).await?;
        info!("authenticated as @{}", me.data.username);

        // --

        Ok(client)
    }

    pub async fn post(&self, post: TextPost) -> Result<()> {
        info!("posting to twitter");

        self.client.post_tweet(&post.content, None).await?;

        Ok(())
    }

    // --

    async fn req<T: DeserializeOwned>(
        &self,
        f: impl AsyncFnOnce(&TweetyClient) -> Result<ResponseWithHeaders, TweetyError>,
    ) -> Result<T> {
        let response = f(&self.client).await?.response;
        trace!("{response}");

        Ok(T::deserialize(response)?)
    }
}

// --

#[derive(Deserialize)]
struct ApiResponse<Data> {
    data: Data,
}
