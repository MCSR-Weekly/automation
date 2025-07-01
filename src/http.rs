use reqwest::Client;
use std::sync::OnceLock;

pub fn get_client() -> Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    fn build_client() -> Client {
        const USER_AGENT: &str = concat!("MCSR-Weekly-Automation/", env!("CARGO_PKG_VERSION"));

        Client::builder()
            .https_only(true)
            .user_agent(USER_AGENT)
            .use_native_tls()
            .build()
            .unwrap()
    }

    CLIENT.get_or_init(build_client).clone()
}
