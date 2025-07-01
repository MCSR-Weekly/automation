mod bsky;
mod discord;
mod mastodon;
mod twitter;

pub use self::bsky::BskyClient;
pub use self::discord::DiscordClient;
pub use self::mastodon::MastodonClient;
pub use self::twitter::TwitterClient;

// --

use crate::Post;
use anyhow::Result;
use log::trace;
use serde::de::DeserializeOwned;

#[allow(async_fn_in_trait)]
pub trait ServiceClient: Sized {
    const NAME: &'static str;

    type Creds: DeserializeOwned;

    async fn new() -> Result<Self> {
        trace!("constructing {} client", Self::NAME);

        let prefix = format!("MWA_{}_", Self::NAME.to_uppercase());
        let creds = envy::prefixed(prefix).from_env::<Self::Creds>()?;

        let mut client = Self::_create(creds).await?;
        trace!("logging into {}", Self::NAME);
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
