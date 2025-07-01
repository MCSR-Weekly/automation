use reqwest::Client;
use std::sync::OnceLock;

/// Get a handle to the global reqwest client
///
/// We use this when building api wrapper clients as we want to set some options
/// for all connections (user agent etc)
pub fn get_http_client() -> Client {
    fn build_client() -> Client {
        const USER_AGENT: &str = concat!("MCSR-Weekly-Automation/", env!("CARGO_PKG_VERSION"));

        Client::builder()
            .https_only(true)
            .user_agent(USER_AGENT)
            .use_native_tls()
            .build()
            .unwrap()
    }

    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(build_client).clone()
}
