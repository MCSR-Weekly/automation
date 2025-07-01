use crate::{SecretString, TextPost, get_http_client};
use anyhow::Result;
use log::info;
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
pub struct MastodonClient {
    client: Mastodon,
}

impl MastodonClient {
    pub async fn new() -> Result<Self> {
        info!("logging in...");

        // -- load credentials --

        #[derive(Deserialize)]
        struct Creds {
            instance: String,
            client_key: SecretString,
            client_secret: SecretString,
            access_token: SecretString,
        }
        let creds = envy::prefixed("MWA_MASTODON_").from_env::<Creds>()?;

        // -- create client --

        let client_data = mastodon_async::Data {
            base: creds.instance.into(),
            client_id: creds.client_key.expose_secret().to_string().into(),
            client_secret: creds.client_secret.expose_secret().to_string().into(),
            redirect: Cow::Borrowed("urn:ietf:wg:oauth:2.0:oob"),
            token: creds.access_token.expose_secret().to_string().into(),
        };
        let client = Mastodon::new(get_http_client(), client_data);

        // -- check login status --

        let app = client.verify_app().await?;
        let account = client.verify_credentials().await?;
        info!("app `{}` authenticated for @{}", app.name, account.username);

        // --

        Ok(MastodonClient { client })
    }

    pub async fn post(&self, post: TextPost) -> Result<()> {
        info!("posting to discord");

        // -- build post --

        let status = NewStatusBuilder::default()
            .status(post.content)
            .visibility(Visibility::Private) // FIXME remove
            .language(Language::Eng) // English
            .build()?;

        // -- submit to api --

        self.client.new_status(status).await?;

        // --

        Ok(())
    }
}
