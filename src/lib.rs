#![cfg_attr(docsrs, feature(doc_cfg))]
//! # cookie_store
//! Provides an implementation for storing and retrieving [`Cookie`]s per the path and domain matching
//! rules specified in [RFC6265](http://tools.ietf.org/html/rfc6265).
//!
//! ## Feature `preserve_order`
//! If enabled, [`CookieStore`] will use [`indexmap::IndexMap`] internally, and [`Cookie`]
//! insertion order will be preserved. Adds dependency `indexmap`.
//!
//! ## Feature `reqwest_impl`
//! If enabled, implementations of the [`reqwest::cookie::CookieStore`] trait are provided. As
//! these are intended for usage in async/concurrent contexts, these implementations use locking
//! primitives [`std::sync::Mutex`] ([`CookieStoreMutex`]) or [`std::sync::RwLock`]
//! ([`CookieStoreRwLock`]).
//!
//! ## Example
//! The following example demonstrates loading a [`CookieStore`] from disk, and using it within a
//! [`CookieStoreMutex`]. It then makes a series of request, examining and modifying the contents
//! of the underlying [`CookieStore`] in between.
//! ```no_run
//! # tokio_test::block_on(async {
//! // Load an existing set of cookies, serialized as json
//! let cookie_store = {
//!   let file = std::fs::File::open("cookies.json")
//!       .map(std::io::BufReader::new)
//!       .unwrap();
//!   cookie_store::CookieStore::load_json(file).unwrap()
//! };
//! let cookie_store = cookie_store::CookieStoreMutex::new(cookie_store);
//! let cookie_store = std::sync::Arc::new(cookie_store);
//! {
//!   // Examine initial contents
//!   println!("initial load");
//!   let store = cookie_store.lock().unwrap();
//!   for c in store.iter_any() {
//!     println!("{:?}", c);
//!   }
//! }
//!
//! // Build a `reqwest` Client, providing the deserialized store
//! let client = reqwest::Client::builder()
//!     .cookie_provider(std::sync::Arc::clone(&cookie_store))
//!     .build()
//!     .unwrap();
//!
//! // Make a sample request
//! client.get("https://google.com").send().await.unwrap();
//! {
//!   // Examine the contents of the store.
//!   println!("after google.com GET");
//!   let store = cookie_store.lock().unwrap();
//!   for c in store.iter_any() {
//!     println!("{:?}", c);
//!   }
//! }
//!
//! // Make another request from another domain
//! println!("GET from msn");
//! client.get("https://msn.com").send().await.unwrap();
//! {
//!   // Examine the contents of the store.
//!   println!("after msn.com GET");
//!   let mut store = cookie_store.lock().unwrap();
//!   for c in store.iter_any() {
//!     println!("{:?}", c);
//!   }
//!   // Clear the store, and examine again
//!   store.clear();
//!   println!("after clear");
//!   for c in store.iter_any() {
//!     println!("{:?}", c);
//!   }
//! }
//!
//! // Get some new cookies
//! client.get("https://google.com").send().await.unwrap();
//! {
//!   // Write store back to disk
//!   let mut writer = std::fs::File::create("cookies2.json")
//!       .map(std::io::BufWriter::new)
//!       .unwrap();
//!   let store = cookie_store.lock().unwrap();
//!   store.save_json(&mut writer).unwrap();
//! }
//! # });
//!```

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
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest_impl")))]
mod reqwest_impl;
#[cfg(feature = "reqwest_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest_impl")))]
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
