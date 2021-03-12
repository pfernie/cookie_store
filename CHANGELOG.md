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
