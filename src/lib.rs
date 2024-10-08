#![cfg_attr(docsrs, feature(doc_cfg))]
//! # cookie_store
//! Provides an implementation for storing and retrieving [`Cookie`]s per the path and domain matching
//! rules specified in [RFC6265](https://datatracker.ietf.org/doc/html/rfc6265).
//!
//! ## Example
//! Please refer to the [reqwest_cookie_store](https://crates.io/crates/reqwest_cookie_store) for
//! an example of using this library along with [reqwest](https://crates.io/crates/reqwest).
//!
//! # Feature flags
//!
//! ## `default`
//! Default features: `[public_suffix, serde_json]`.
//!
//! ## `preserve_order`
//! If enabled, [`CookieStore`] will use [`indexmap::IndexMap`] internally, and [`Cookie`]
//! insertion order will be preserved. Adds dependency `indexmap`.
//!
//! ## `public_suffix`
//! __Enabled by default__. Add support for public suffix lists, as provided by
//! [publicsuffix](https://crates.io/crates/publicsuffix).
//!
//! ## `wasm-bindgen`
//! Enabled transitive feature `time/wasm-bindgen`; necessary in `wasm` contexts.
//!
//! ## `serde`
//! Supports generic (format-agnostic) de/serialization for a `CookieStore`. Adds dependencies `serde` and `serde_derive`.
//!
//! ## `serde_json`
//! __Enabled by default__. Supports de/serialization for a `CookieStore` via the JSON format. Enables
//! feature `serde` and adds depenency `serde_json`.
//!
//! ## `serde_ron`
//! Supports de/serialization for a `CookieStore` via the RON format. Enables
//! feature `serde` and adds depenency `ron`.
//!
//! ## `log_secure_cookie_values`
//! Enable logging the values of cookies marked 'secure', off by default as values may be sensitive

use idna;

pub use ::cookie::{Cookie as RawCookie, ParseError as RawCookieParseError};

mod cookie;
pub use crate::cookie::Error as CookieError;
pub use crate::cookie::{Cookie, CookieResult};
mod cookie_domain;
pub use crate::cookie_domain::CookieDomain;
mod cookie_expiration;
pub use crate::cookie_expiration::CookieExpiration;
mod cookie_path;
pub use crate::cookie_path::CookiePath;
mod cookie_store;
pub use crate::cookie_store::{CookieStore, StoreAction};
#[cfg(feature = "serde")]
pub mod serde;
mod utils;

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

#[cfg(feature = "serde")]
pub(crate) mod rfc3339_fmt {

    pub(crate) const RFC3339_FORMAT: &[time::format_description::FormatItem] =
        time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

    pub(super) fn serialize<S>(t: &time::OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        // An explicit format string is used here, instead of time::format_description::well_known::Rfc3339, to explicitly
        // utilize the 'Z' terminator instead of +00:00 format for Zulu time.
        let s = t.format(&RFC3339_FORMAT).map_err(|e| {
            println!("{}", e);
            S::Error::custom(format!(
                "Could not parse datetime '{}' as RFC3339 UTC format: {}",
                t, e
            ))
        })?;
        serializer.serialize_str(&s)
    }

    pub(super) fn deserialize<'de, D>(t: D) -> Result<time::OffsetDateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::{de::Error, Deserialize};

        let s = String::deserialize(t)?;
        time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339).map_err(
            |e| {
                D::Error::custom(format!(
                    "Could not parse string '{}' as RFC3339 UTC format: {}",
                    s, e
                ))
            },
        )
    }
}
