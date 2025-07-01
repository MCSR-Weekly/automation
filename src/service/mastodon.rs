use crate::{PostContent, SecretString, credentials, http};
use anyhow::Result;
use mastodon_async::{Language, Mastodon, NewStatusBuilder, Visibility};
use serde::Deserialize;
use std::borrow::Cow;
use tokio::sync::OnceCell;

#[derive(Debug, Deserialize)]
pub struct MastodonCredentials {
    instance: String,
    client_key: SecretString,
    client_secret: SecretString,
    access_token: SecretString,
}

// --

const ENGLISH: Language = Language::Eng;

pub async fn post(post: PostContent) -> Result<()> {
    info!("posting to mastodon");

    let client = get_client().await?;

    let status = NewStatusBuilder::default()
        .status(post.raw_text)
        .visibility(Visibility::Private) // FIXME remove
        .language(ENGLISH)
        .build()?;
    client.new_status(status).await?;

    Ok(())
}

// --

async fn get_client() -> Result<Mastodon> {
    async fn build_client() -> Result<Mastodon> {
        let creds = &credentials::get().mastodon;
        let data = mastodon_async::Data {
            base: Cow::Borrowed(&creds.instance),
            client_id: Cow::Borrowed(creds.client_key.expose_secret()),
            client_secret: Cow::Borrowed(creds.client_secret.expose_secret()),
            redirect: Cow::Borrowed("urn:ietf:wg:oauth:2.0:oob"),
            token: Cow::Borrowed(creds.access_token.expose_secret()),
        };
        let reqwest = http::get_client();
        let client = Mastodon::new(reqwest, data);

        let app = client.verify_app().await?;
        let account = client.verify_credentials().await?;
        info!("app `{}` authenticated for @{}", app.name, account.username);

        Ok(client)
    }

    static CLIENT: OnceCell<Mastodon> = OnceCell::const_new();
    Ok(CLIENT.get_or_try_init(build_client).await?.clone())
}
