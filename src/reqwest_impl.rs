use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use bytes::Bytes;
use cookie::{Cookie as RawCookie, ParseError as RawCookieParseError};
use reqwest::header::HeaderValue;

use crate::CookieStore;

// We provide impls of the methods to support the [`reqwest::cookie::CookieStore`] trait,
// but do not directly implement the trait as our `set_cookies` must take `&mut self`.
impl CookieStore {
    fn set_cookies(
        &mut self,
        cookie_headers: &mut dyn Iterator<Item = &HeaderValue>,
        url: &url::Url,
    ) {
        let cookies = cookie_headers.filter_map(|val| {
            std::str::from_utf8(val.as_bytes())
                .map_err(RawCookieParseError::from)
                .and_then(RawCookie::parse)
                .map(|c| c.into_owned())
                .ok()
        });
        self.store_response_cookies(cookies, url);
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        let s = self
            .get_request_cookies(url)
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");

        if s.is_empty() {
            return None;
        }

        HeaderValue::from_maybe_shared(Bytes::from(s)).ok()
    }
}

/// A [`CookieStore`] wrapped internally by a [`std::sync::Mutex`], suitable for use in
/// async/concurrent contexts.
#[derive(Debug)]
pub struct CookieStoreMutex(Mutex<CookieStore>);

impl Default for CookieStoreMutex {
    /// Create a new, empty [`CookieStoreMutex`].
    fn default() -> Self {
        CookieStoreMutex::new(CookieStore::default())
    }
}

impl CookieStoreMutex {
    /// Create a new [`CookieStoreMutex`] from an existing [`CookieStore`].
    pub fn new(cookie_store: CookieStore) -> CookieStoreMutex {
        CookieStoreMutex(Mutex::new(cookie_store))
    }

    /// Lock and get a handle to the contained [`CookieStore`].
    pub fn lock(&self) -> Result<MutexGuard<CookieStore>, PoisonError<MutexGuard<CookieStore>>> {
        self.0.lock()
    }
}

impl reqwest::cookie::CookieStore for CookieStoreMutex {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        let mut store = self.0.lock().unwrap();
        store.set_cookies(cookie_headers, url);
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        let store = self.0.lock().unwrap();
        store.cookies(url)
    }
}

/// A [`CookieStore`] wrapped internally by a [`std::sync::RwLock`], suitable for use in
/// async/concurrent contexts.
#[derive(Debug)]
pub struct CookieStoreRwLock(RwLock<CookieStore>);

impl Default for CookieStoreRwLock {
    /// Create a new, empty [`CookieStoreRwLock`].
    fn default() -> Self {
        CookieStoreRwLock::new(CookieStore::default())
    }
}

impl CookieStoreRwLock {
    /// Create a new [`CookieStoreRwLock`] from an existing [`CookieStore`].
    pub fn new(cookie_store: CookieStore) -> CookieStoreRwLock {
        CookieStoreRwLock(RwLock::new(cookie_store))
    }

    /// Lock and get a read (non-exclusive) handle to the contained [`CookieStore`].
    pub fn read(
        &self,
    ) -> Result<RwLockReadGuard<CookieStore>, PoisonError<RwLockReadGuard<CookieStore>>> {
        self.0.read()
    }

    /// Lock and get a write (exclusive) handle to the contained [`CookieStore`].
    pub fn write(
        &self,
    ) -> Result<RwLockWriteGuard<CookieStore>, PoisonError<RwLockWriteGuard<CookieStore>>> {
        self.0.write()
    }
}

impl reqwest::cookie::CookieStore for CookieStoreRwLock {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        let mut write = self.0.write().unwrap();
        write.set_cookies(cookie_headers, url);
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        let read = self.0.read().unwrap();
        read.cookies(url)
    }
}
