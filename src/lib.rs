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
mod reqwest_impl {
    use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

    use crate::CookieStore;

    /// A `CookieStore` wrapped internally by a `std::sync::Mutex`, suitable for use in
    /// async/concurrent contexts.
    #[derive(Debug)]
    pub struct CookieStoreMutex(Mutex<CookieStore>);

    impl CookieStoreMutex {
        pub fn new(cookie_store: CookieStore) -> CookieStoreMutex {
            CookieStoreMutex(Mutex::new(cookie_store))
        }

        pub fn lock(
            &self,
        ) -> Result<MutexGuard<CookieStore>, PoisonError<MutexGuard<CookieStore>>> {
            self.0.lock()
        }
    }

    impl reqwest::cookie::CookieStore for CookieStoreMutex {
        fn set_cookies(&self, cookie_headers: Vec<&str>, url: &url::Url) {
            let mut store = self.0.lock().unwrap();
            for cookie in cookie_headers {
                let _ = store.parse(cookie, url);
            }
        }

        fn cookies(&self, url: &url::Url) -> Vec<String> {
            let store = self.0.lock().unwrap();
            store
                .matches(url)
                .into_iter()
                .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
                .collect()
        }
    }

    /// A `CookieStore` wrapped internally by a `std::sync::RwLock`, suitable for use in
    /// async/concurrent contexts.
    #[derive(Debug)]
    pub struct CookieStoreRwLock(RwLock<CookieStore>);

    impl CookieStoreRwLock {
        pub fn new(cookie_store: CookieStore) -> CookieStoreRwLock {
            CookieStoreRwLock(RwLock::new(cookie_store))
        }

        pub fn read(
            &self,
        ) -> Result<RwLockReadGuard<CookieStore>, PoisonError<RwLockReadGuard<CookieStore>>>
        {
            self.0.read()
        }

        pub fn write(
            &self,
        ) -> Result<RwLockWriteGuard<CookieStore>, PoisonError<RwLockWriteGuard<CookieStore>>>
        {
            self.0.write()
        }
    }

    impl reqwest::cookie::CookieStore for CookieStoreRwLock {
        fn set_cookies(&self, cookie_headers: Vec<&str>, url: &url::Url) {
            let mut write = self.0.write().unwrap();
            for cookie in cookie_headers {
                let _ = write.parse(cookie, url);
            }
        }

        fn cookies(&self, url: &url::Url) -> Vec<String> {
            let read = self.0.read().unwrap();
            read.matches(url)
                .into_iter()
                .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
                .collect()
        }
    }
}
#[cfg(feature = "reqwest_impl")]
pub use reqwest_impl::{CookieStoreMutex, CookieStoreRwLock};

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
