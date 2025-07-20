[![Build Status](https://github.com/pfernie/cookie_store/actions/workflows/ci.yml/badge.svg)](https://github.com/pfernie/cookie_store/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/cookie_store/badge.svg)](https://docs.rs/cookie_store)

Provides an implementation for storing and retrieving `Cookie`s per the path and domain matching 
rules specified in [RFC6265](https://datatracker.ietf.org/doc/html/rfc6265).

## Features

* `preserve_order` - uses `indexmap::IndexMap` in lieu of HashMap internally, so cookies are maintained in insertion/creation order
* `public_suffix` - Add support for public suffix lists, as provided by [publicsuffix](https://crates.io/crates/publicsuffix).
* `wasm-bindgen` - Enables transitive feature `time/wasm-bindgen`; necessary in `wasm` contexts.
* `log_secure_cookie_values` - Enable logging the values of cookies marked 'secure', off by default as values may be sensitive

### Serialization
* `serde` - Supports generic (format-agnostic) de/serialization for a `CookieStore`. Adds dependencies `serde` and `serde_derive`.
* `serde_json` - Supports de/serialization for a `CookieStore` via the JSON format. Enables feature `serde` and adds depenency `serde_json`.
* `serde_ron` - Supports de/serialization for a `CookieStore` via the RON format. Enables feature `serde` and adds depenency `ron`.

## Usage with [reqwest](https://crates.io/crates/reqwest)

Please refer to the [reqwest_cookie_store](https://crates.io/crates/reqwest_cookie_store) crate, which now provides an implementation of the `reqwest::cookie::CookieStore` trait for `cookie_store::CookieStore`.

## License
This project is licensed and distributed under the terms of both the MIT license and Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT)
