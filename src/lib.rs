use idna;

mod cookie;
pub use crate::cookie::Error as CookieError;
pub use crate::cookie::{Cookie, CookieResult};
mod cookie_domain;
mod cookie_expiration;
mod cookie_path;
mod cookie_store;
pub use crate::cookie_store::CookieStore;
mod utils;

#[cfg(feature = "reqwest_impl")]
impl reqwest::cookie::CookieStore for CookieStore {
    fn store_cookie_headers(&mut self, cookie_headers: Vec<&str>, url: &url::Url) {
        for cookie in cookie_headers {
            let _ = self.parse(cookie, url);
        }
    }

    fn get_cookie_headers(&self, url: &url::Url) -> Vec<String> {
        self.matches(url)
            .into_iter()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct IdnaErrors(idna::Errors);

impl std::fmt::Display for IdnaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IDNA errors: {:#?}", self.0)
    }
}

impl std::error::Error for IdnaErrors {}

impl From<idna::Errors> for IdnaErrors {
    fn from(e: idna::Errors) -> Self {
        IdnaErrors(e)
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) mod rfc3339_fmt {
    use serde::{de::Error, Deserialize};

    pub(crate) const RFC3339_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";
    pub(super) fn serialize<S>(t: &time::OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // An explicit format string is used here, instead of time::Format::Rfc3339, to explicitly
        // utilize the 'Z' terminator instead of +00:00 format for Zulu time.
        let s = t.format(RFC3339_FORMAT);
        serializer.serialize_str(&s)
    }

    pub(super) fn deserialize<'de, D>(t: D) -> Result<time::OffsetDateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(t)?;
        time::OffsetDateTime::parse(&s, time::Format::Rfc3339).map_err(|e| {
            D::Error::custom(format!(
                "Could not parse string '{}' as RFC3339 UTC format: {}",
                s, e
            ))
        })
    }
}
