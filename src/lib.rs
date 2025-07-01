//! mcsr weekly automation tooling

mod http;

mod post;
mod secret;
mod service;

pub use self::http::get_http_client;
pub use self::post::{Post, RichPost, TextPost};
pub use self::secret::SecretString;
pub use self::service::{BskyClient, DiscordClient, MastodonClient, TwitterClient};
