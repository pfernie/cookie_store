# Changelog

## [0.16.2] - 2023-06-16

### Ci

- Backport cliff.toml & release.sh to v0.16.X branch
- Allow serde and serde_derive to compile in parallel

= v0.16.1 =
* Export `cookie_domain::CookieDomain` as `pub`
* Export `pub use cookie_expiration::CookieExpiration`
* Export `pub use cookie_path::CookiePath`
* Make `CookieStore::from_cookies` pub
* Add methods `CookieStore::load_json_all` and `CookieStore::load_all` to allow
  for loading both __unexpired__ and __expired__ cookies.


= v0.16.0 =
* Update of dependencies in public API in `0.15.2` should have qualified as minor version bump

= v0.15.2 = __YANKED__
* Upgrade dependencies

= v0.15.1 =
* Attach `Secure` cookies to requests for `http://localhost` and loopback IP addresses (e.g. `127.0.0.1`). This change aligns `cookie_store`'s behaviour to the behaviour of [Chromium-based browsers](https://bugs.chromium.org/p/chromium/issues/detail?id=1177877#c7) and [Firefox](https://hg.mozilla.org/integration/autoland/rev/c4d13b3ca1e2).
  
= v0.15.0 =
* deprecation in `v0.14.1` should have qualified as minor version bump
* Upgrade dependencies

= v0.14.1 =
* Improve documentation on `CookieStore::get_request_cookies`
* Introduce alternative `CookieStore::get_request_values`, mark `CookieStore::get_request_cookies` as deprecated, and suggest usage of `get_request_values` instead.

= v0.14.0 =
* **BREAKING** The `CookieStoreMutex` and `CookieStoreRwLock` implementation previously provided under the `reqwest_impl` feature have been migrated to a dedicated crate, `reqwest_cookie_store`, and the feature has been removed.
* **BREAKING** `reqwest` is no longer a direct depdency, but rather a `dev-depedency`. Furthermore, now only the needed `reqwest` features (`cookies`) are enabled, as opposed to all default features. This is potentially a breaking change for users.
* `reqwest` is no longer an optional dependency, it is now a `dev-dependency` for doctests.
  * Only enable the needed features for `reqwest` (@blyxxyz)
* Upgrade `publisuffix` dependency to `v2` (@rushmorem)
* Remove unused dev-dependencies

= v0.13.3 =
* Fix attributes & configuration for feature support in docs.rs

= v0.13.0 =
* Introduce optional feature `reqwest_impl`, providing implementations of the `reqwest::cookie::CookieStore` trait
* Upgrade to `reqwest 0.11.2`
* Upgrade to `env_logger 0.8`
* Upgrade to `pretty_assertions 0.7`
* Upgrade to `cookie 0.15`

= v0.12.0 =
* Upgrade to `cookie 0.14`
* Upgrade to `time 0.2`

= v0.11.0 =
* Implement `{De,}Serialize` for `CookieStore` (@Felerius)
  
= v0.10.0 =
* introduce optional feature `preserve_order` which maintains cookies in insertion order.

= v0.9.0 =
* remove `try_from` dependency again now that `reqwest` minimum rust version is bumped
* upgrade to `url 2.0` (@benesch)
* Upgrade to `idna 0.2`

= v0.8.0 =
* Remove dependency on `failure` (seanmonstar)

= v0.7.0 =
* Revert removal of `try_from` dependency

= v0.6.0 =
* Upgrades to `cookies` v0.12
* Drop dependency `try_from` in lieu of `std::convert::TryFrom` (@oherrala)
* Drop dependency on `serde_derive`, rely on `serde` only (@oherrala)

= v0.4.0 =
* Update to Rust 2018 edition

= v0.3.1 =

* Upgrades to `cookies` v0.11
* Minor dependency upgrades

= v0.3 =

* Upgrades to `reqwest` v0.9
* Replaces `error-chain` with `failure`

= v0.2 =

* Removes separate `ReqwestSession::ErrorKind`. Added as variant `::ErrorKind::Reqwest` instead.
