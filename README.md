[![Build Status](https://travis-ci.org/pfernie/cookie_store.svg?branch=master)](https://travis-ci.org/pfernie/cookie_store)
[![Documentation](https://docs.rs/cookie_store/badge.svg)](https://docs.rs/cookie_store)

Provides an implementation for storing and retrieving `Cookie`s per the path and domain matching 
rules specified in [RFC6265](http://tools.ietf.org/html/rfc6265).

Split from the [user_agent](https://github.com/pfernie/user_agent) crate.

## Features

* `preserve_order` - if enabled, iteration order of cookies will be maintained in insertion order. Pulls in an additional dependency on the [indexmap](https://crates.io/crates/indexmap) crate.
* `reqwest_impl` - if enabled, implementations of the [`reqwest::cookie::CookieStore`](https://github.com/seanmonstar/reqwest/blob/12d7905520fee4cc96ca5e5a6d1fc523802cafc3/src/cookie.rs#L12) trait are provided, as `CookieStoreMutex` or `CookieStoreRwLock`.

## License
This project is licensed and distributed under the terms of both the MIT license and Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT)
