use serde::Deserialize;
use std::fmt;

// not debug printed
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub(crate) struct SecretString {
    inner: String,
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<secret>")
    }
}

impl SecretString {
    #[inline]
    #[must_use]
    pub(crate) fn expose_secret(&self) -> &str {
        &self.inner
    }
}
