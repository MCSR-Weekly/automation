use serde::Deserialize;
use std::fmt;
use std::sync::Arc;

// not debug printed
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct SecretString {
    inner: Arc<str>,
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<secret>")
    }
}

impl SecretString {
    #[inline]
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        &self.inner
    }
}
