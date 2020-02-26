mod cookie;
mod cookie_domain;
mod cookie_expiration;
mod cookie_path;
mod cookie_store;
mod utils;

pub use crate::cookie::{Cookie, CookieError, CookieResult};
pub use crate::cookie_store::CookieStore;

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
