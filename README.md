[![Build Status](https://travis-ci.org/pfernie/cookie_store.svg?branch=master)](https://travis-ci.org/pfernie/cookie_store)
[![Gitter chat](https://badges.gitter.im/gitterHQ/gitter.png)](https://gitter.im/user_agent)

[Documentation](https://docs.rs/cookie_store/)

Provides an implementation for storing and retrieving `Cookie`s per the path and domain matching 
rules specified in [RFC6265](http://tools.ietf.org/html/rfc6265).

Split from the [user_agent](https://github.com/pfernie/user_agent) crate.

## Features

* `preserve_order` - if enabled, iteration order of cookies will be maintained in insertion order. Pulls in an additional dependency on the [indexmap](https://crates.io/crates/indexmap) crate.
* `reqwest_impl` - provide an implementation of `reqwest::cookies::CookieStore` trait in two flavors: `CookieStoreMutex` and `CookieStoreRwLock`.

## License
This project is licensed and distributed under the terms of both the MIT license and Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT)
