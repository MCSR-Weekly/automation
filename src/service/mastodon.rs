use crate::{SecretString, ServiceClient, TextPost, get_http_client};
use anyhow::Result;
use mastodon_async::{Language, Mastodon, NewStatusBuilder, Visibility};
use serde::Deserialize;
use std::borrow::Cow;

/// Mastodon client
///
/// # Credentials
/// - `MWA_MASTODON_INSTANCE`
/// - `MWA_MASTODON_CLIENT_KEY`
/// - `MWA_MASTODON_CLIENT_SECRET`
/// - `MWA_MASTODON_ACCESS_TOKEN`
pub(crate) struct MastodonClient {
    client: Mastodon,
}

#[derive(Deserialize)]
pub(crate) struct Creds {
    instance: String,
    client_key: SecretString,
    client_secret: SecretString,
    access_token: SecretString,
}

impl ServiceClient for MastodonClient {
    const NAME: &'static str = "mastodon";
    type Creds = Creds;

    async fn _create(creds: Creds) -> Result<Self> {
        let client_data = mastodon_async::Data {
            base: creds.instance.into(),
            client_id: creds.client_key.expose_secret().to_string().into(),
            client_secret: creds.client_secret.expose_secret().to_string().into(),
            redirect: Cow::Borrowed("urn:ietf:wg:oauth:2.0:oob"),
            token: creds.access_token.expose_secret().to_string().into(),
        };
        let client = Mastodon::new(get_http_client(), client_data);

        Ok(MastodonClient { client })
    }

    async fn _login(&mut self) -> Result<()> {
        let app = self.client.verify_app().await?;
        let account = self.client.verify_credentials().await?;
        info!(
            "successfully authenticated as @{}@{} via app `{}`",
            account.username,
            account.url.domain().unwrap(), // this is so dumb i hate it
            app.name,
        );

        Ok(())
    }

    type CreatePostInput = TextPost;
    async fn create_post(&self, post: TextPost) -> Result<()> {
        let status = NewStatusBuilder::default()
            .status(post.content)
            .visibility(Visibility::Private) // FIXME remove
            .language(Language::Eng) // English
            .build()?;
        self.client.new_status(status).await?;

        Ok(())
    }
}
