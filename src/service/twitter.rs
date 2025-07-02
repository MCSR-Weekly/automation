use crate::{SecretString, ServiceClient, TextPost};
use anyhow::Result;
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
pub(crate) struct TwitterClient {
    client: TweetyClient,
}

#[derive(Deserialize)]
pub(crate) struct Creds {
    api_key: SecretString,
    api_secret: SecretString,
    access_token: SecretString,
    access_secret: SecretString,
}

impl ServiceClient for TwitterClient {
    const NAME: &'static str = "twitter";
    type Creds = Creds;

    async fn _create(creds: Creds) -> Result<Self> {
        let client = TweetyClient::new(
            creds.api_key.expose_secret(),
            creds.access_token.expose_secret(),
            creds.api_secret.expose_secret(),
            creds.access_secret.expose_secret(),
        );

        Ok(TwitterClient { client })
    }

    async fn _login(&mut self) -> Result<()> {
        #[derive(Deserialize)]
        struct CurrentUser {
            // id: String,
            // name: String,
            username: String,
        }
        let me: ApiResponse<CurrentUser> = self.req(async |c| c.get_user_me(None).await).await?;
        info!("successfully authenticated as @{}", me.data.username);

        Ok(())
    }

    type CreatePostInput = TextPost;
    async fn create_post(&self, post: TextPost) -> Result<()> {
        self.client.post_tweet(&post.content, None).await?;

        Ok(())
    }
}

// -- api util --

impl TwitterClient {
    async fn req<T: DeserializeOwned>(
        &self,
        f: impl AsyncFnOnce(&TweetyClient) -> Result<ResponseWithHeaders, TweetyError>,
    ) -> Result<T> {
        let response = f(&self.client).await?.response;
        trace!("{response}"); // FIXME

        Ok(T::deserialize(response)?)
    }
}

#[derive(Deserialize)]
struct ApiResponse<Data> {
    data: Data,
}
