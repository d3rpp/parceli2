use std::fmt::Display;

pub mod api;
pub mod local;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
#[repr(transparent)]
pub struct OString {
    inner: Option<String>,
}

impl OString {
    pub fn new(inner: Option<impl ToString>) -> OString {
        OString {
            inner: inner.map(|value| value.to_string()),
        }
    }

    pub fn inner(self) -> Option<String> {
        self.inner
    }

    pub fn is_some(&self) -> bool {
        self.inner.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }
}

impl Display for OString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Some(item) => write!(f, "{}", item),
            None => write!(f, "Unknown"),
        }
    }
}
