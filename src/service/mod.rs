mod bsky;
mod discord;
mod mastodon;
mod twitter;

pub(crate) use self::bsky::BskyClient;
pub(crate) use self::discord::DiscordClient;
pub(crate) use self::mastodon::MastodonClient;
pub(crate) use self::twitter::TwitterClient;

// --

use crate::Post;
use anyhow::Result;
use serde::de::DeserializeOwned;

#[allow(async_fn_in_trait)]
pub(crate) trait ServiceClient: Sized {
    const NAME: &'static str;

    type Creds: DeserializeOwned;

    async fn new() -> Result<Self> {
        trace!("authenticating with {}", Self::NAME);

        let prefix = format!("MWA_{}_", Self::NAME.to_uppercase());
        let creds = envy::prefixed(prefix).from_env::<Self::Creds>()?;

        let mut client = Self::_create(creds).await?;
        client._login().await?;
        Ok(client)
    }

    async fn _create(creds: Self::Creds) -> Result<Self>;
    async fn _login(&mut self) -> Result<()>;

    /// either `TextPost` or `RichPost`
    type CreatePostInput: From<Post>;
    /// post that shit!
    async fn create_post(&self, post: Self::CreatePostInput) -> Result<()>;
}
