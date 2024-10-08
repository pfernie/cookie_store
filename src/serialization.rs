use serde_derive::{Serialize, Deserialize};
use std::io::{BufRead, Write};

use crate::{Cookie, cookie_store::StoreResult, CookieStore};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CookieStoreSerialized<'a> {
    cookies: Vec<Cookie<'a>>,
}

/// Load cookies from `reader`, deserializing with `cookie_from_str`, skipping any __expired__
/// cookies
pub fn load<R, E, F>(reader: R, cookies_from_str: F) -> StoreResult<CookieStore>
    where
    R: BufRead,
    F: Fn(&str) -> Result<CookieStoreSerialized<'static>, E>,
    crate::Error: From<E>,
{
    load_from(reader, cookies_from_str, false)
}

/// Load cookies from `reader`, deserializing with `cookie_from_str`, loading both __unexpired__
/// and __expired__ cookies
pub fn load_all<R, E, F>(reader: R, cookies_from_str: F) -> StoreResult<CookieStore>
    where
    R: BufRead,
    F: Fn(&str) -> Result<CookieStoreSerialized<'static>, E>,
    crate::Error: From<E>,
{
    load_from(reader, cookies_from_str, true)
}

fn load_from<R, E, F>(
    mut reader: R,
    cookies_from_str: F,
    include_expired: bool,
) -> StoreResult<CookieStore>
    where
    R: BufRead,
    F: Fn(&str) -> Result<CookieStoreSerialized<'static>, E>,
    crate::Error: From<E>,
{
    let mut cookie_store = String::new();
    reader.read_to_string(&mut cookie_store)?;
    let cookie_store: CookieStoreSerialized = cookies_from_str(&cookie_store)?;
    CookieStore::from_cookies(
        cookie_store.cookies.into_iter().map(|cookies| Ok(cookies)),
        include_expired,
    )
}

/// Load JSON-formatted cookies from `reader`, skipping any __expired__ cookies
#[cfg(feature = "serde_json")]
pub fn load_json<R: BufRead>(reader: R) -> StoreResult<CookieStore> {
    load(reader, |cookies| serde_json::from_str(cookies))
}

/// Load JSON-formatted cookies from `reader`, loading both __expired__ and __unexpired__ cookies
#[cfg(feature = "serde_json")]
pub fn load_json_all<R: BufRead>(reader: R) -> StoreResult<CookieStore> {
    load_all(reader, |cookies| serde_json::from_str(cookies))
}

/// Load RON-formatted cookies from `reader`, skipping any __expired__ cookies
#[cfg(feature = "serde_ron")]
pub fn load_ron<R: BufRead>(reader: R) -> StoreResult<CookieStore> {
    load(reader, |cookies| ron::from_str(cookies))
}

/// Load RON-formatted cookies from `reader`, loading both __expired__ and __unexpired__ cookies
#[cfg(feature = "serde_ron")]
pub fn load_ron_all<R: BufRead>(reader: R) -> StoreResult<CookieStore> {
    load_all(reader, |cookies| ron::from_str(cookies))
}

/// Serialize any __unexpired__ and __persistent__ cookies in the store with `cookie_to_string`
/// and write them to `writer`
pub fn save<W, E, F>(
    cookie_store: &CookieStore,
    writer: &mut W,
    cookies_to_string: F,
) -> StoreResult<()>
    where
    W: Write,
    F: Fn(&CookieStoreSerialized<'static>) -> Result<String, E>,
    crate::Error: From<E>,
{
    let mut cookies = Vec::new();
    for cookie in cookie_store.iter_unexpired() {
        if cookie.is_persistent() {
            cookies.push(cookie.clone());
        }
    }
    let cookie_store = CookieStoreSerialized { cookies };
    let cookies = cookies_to_string(&cookie_store);
    writeln!(writer, "{}", cookies?)?;
    Ok(())
}

/// Serialize any __unexpired__ and __persistent__ cookies in the store to JSON format and
/// write them to `writer`
#[cfg(feature = "serde_json")]
pub fn save_json<W: Write>(cookie_store: &CookieStore, writer: &mut W) -> StoreResult<()> {
    save(cookie_store, writer, ::serde_json::to_string_pretty)
}

/// Serialize any __unexpired__ and __persistent__ cookies in the store to JSON format and
/// write them to `writer`
#[cfg(feature = "serde_ron")]
pub fn save_ron<W: Write>(cookie_store: &CookieStore, writer: &mut W) -> StoreResult<()> {
    save(cookie_store, writer, |string| {
        ::ron::ser::to_string_pretty(string, ron::ser::PrettyConfig::default())
    })
}

/// Serialize all (including __expired__ and __non-persistent__) cookies in the store with `cookie_to_string` and write them to `writer`
pub fn save_incl_expired_and_nonpersistent<W, E, F>(
    cookie_store: &CookieStore,
    writer: &mut W,
    cookies_to_string: F,
) -> StoreResult<()>
    where
    W: Write,
    F: Fn(&CookieStoreSerialized<'static>) -> Result<String, E>,
    crate::Error: From<E>,
{
    let mut cookies = Vec::new();
    for cookie in cookie_store.iter_any() {
        cookies.push(cookie.clone());
    }
    let cookie_store = CookieStoreSerialized { cookies };
    let cookies = cookies_to_string(&cookie_store);
    writeln!(writer, "{}", cookies?)?;
    Ok(())
}

/// Serialize all (including __expired__ and __non-persistent__) cookies in the store to JSON format and write them to `writer`
#[cfg(feature = "serde_json")]
pub fn save_incl_expired_and_nonpersistent_json<W: Write>(
    cookie_store: &CookieStore,
    writer: &mut W,
) -> StoreResult<()> {
    save_incl_expired_and_nonpersistent(cookie_store, writer, ::serde_json::to_string_pretty)
}

/// Serialize all (including __expired__ and __non-persistent__) cookies in the store to RON format and write them to `writer`
#[cfg(feature = "serde_ron")]
pub fn save_incl_expired_and_nonpersistent_ron<W: Write>(
    cookie_store: &CookieStore,
    writer: &mut W,
) -> StoreResult<()> {
    save_incl_expired_and_nonpersistent(cookie_store, writer, |string| {
        ::ron::ser::to_string_pretty(string, ron::ser::PrettyConfig::default())
    })
}

#[cfg(all(test, feature = "serde_json"))]
mod tests_json {
    use std::io::BufWriter;

    use super::{ save_incl_expired_and_nonpersistent_json, save_json };

    use super::{ load_json, load_json_all };

    fn cookie_json() -> String {
        r#"{
  "cookies": [
    {
      "raw_cookie": "2=two; SameSite=None; Secure; Path=/; Expires=Tue, 03 Aug 2100 00:38:37 GMT",
      "path": [
        "/",
        true
      ],
      "domain": {
        "HostOnly": "test.com"
      },
      "expires": {
        "AtUtc": "2100-08-03T00:38:37Z"
      }
    }
  ]
}
"#
            .to_string()
    }

    fn cookie_json_expired() -> String {
        r#"{
  "cookies": [
    {
      "raw_cookie": "1=one; SameSite=None; Secure; Path=/; Expires=Thu, 03 Aug 2000 00:38:37 GMT",
      "path": [
        "/",
        true
      ],
      "domain": {
        "HostOnly": "test.com"
      },
      "expires": {
        "AtUtc": "2000-08-03T00:38:37Z"
      }
    }
  ]
}
"#
            .to_string()
    }

    #[test]
    fn check_count_json() {
        let cookie = cookie_json();

        let cookie_store = load_json(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store.iter_any().map(|_| 1).sum::<i32>(), 1);
        assert_eq!(cookie_store.iter_unexpired().map(|_| 1).sum::<i32>(), 1);

        let cookie_store_all = load_json_all(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store_all.iter_any().map(|_| 1).sum::<i32>(), 1);
        assert_eq!(cookie_store_all.iter_unexpired().map(|_| 1).sum::<i32>(), 1);

        let mut writer = BufWriter::new(Vec::new());
        save_json(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_json(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);

        let mut writer = BufWriter::new(Vec::new());
        save_json(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_json(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);
    }

    #[test]
    fn check_count_json_expired() {
        let cookie = cookie_json_expired();

        let cookie_store = load_json(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store.iter_any().map(|_| 1).sum::<i32>(), 0);
        assert_eq!(cookie_store.iter_unexpired().map(|_| 1).sum::<i32>(), 0);

        let cookie_store_all = load_json_all(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store_all.iter_any().map(|_| 1).sum::<i32>(), 1);
        assert_eq!(cookie_store_all.iter_unexpired().map(|_| 1).sum::<i32>(), 0);

        let mut writer = BufWriter::new(Vec::new());
        save_json(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!("{\n  \"cookies\": []\n}\n", string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_json(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!("{\n  \"cookies\": []\n}\n", string);

        let mut writer = BufWriter::new(Vec::new());
        save_json(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!("{\n  \"cookies\": []\n}\n", string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_json(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);
    }
}

#[cfg(all(test, feature = "serde_ron"))]
mod tests_ron {
    use std::io::BufWriter;

    use super::{load_ron, load_ron_all};
    use super::{ save_incl_expired_and_nonpersistent_ron, save_ron };

    fn cookie_ron() -> String {
        r#"(
    cookies: [
        (
            raw_cookie: "2=two; SameSite=None; Secure; Path=/; Expires=Tue, 03 Aug 2100 00:38:37 GMT",
            path: ("/", true),
            domain: HostOnly("test.com"),
            expires: AtUtc("2100-08-03T00:38:37Z"),
        ),
    ],
)
"#.to_string()
    }

    fn cookie_ron_expired() -> String {
        r#"(
    cookies: [
        (
            raw_cookie: "1=one; SameSite=None; Secure; Path=/; Expires=Thu, 03 Aug 2000 00:38:37 GMT",
            path: ("/", true),
            domain: HostOnly("test.com"),
            expires: AtUtc("2000-08-03T00:38:37Z"),
        ),
    ],
)
"#.to_string()
    }

    #[test]
    fn check_count_ron() {
        let cookie = cookie_ron();

        let cookie_store = load_ron(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store.iter_any().map(|_| 1).sum::<i32>(), 1);
        assert_eq!(cookie_store.iter_unexpired().map(|_| 1).sum::<i32>(), 1);

        let cookie_store_all = load_ron_all(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store_all.iter_any().map(|_| 1).sum::<i32>(), 1);
        assert_eq!(cookie_store_all.iter_unexpired().map(|_| 1).sum::<i32>(), 1);

        let mut writer = BufWriter::new(Vec::new());
        save_ron(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_ron(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);

        let mut writer = BufWriter::new(Vec::new());
        save_ron(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_ron(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);
    }

    #[test]
    fn check_count_ron_expired() {
        let cookie = cookie_ron_expired();

        let cookie_store = load_ron(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store.iter_any().map(|_| 1).sum::<i32>(), 0);
        assert_eq!(cookie_store.iter_unexpired().map(|_| 1).sum::<i32>(), 0);

        let cookie_store_all = load_ron_all(Into::<&[u8]>::into(cookie.as_bytes())).unwrap();
        assert_eq!(cookie_store_all.iter_any().map(|_| 1).sum::<i32>(), 1);
        assert_eq!(cookie_store_all.iter_unexpired().map(|_| 1).sum::<i32>(), 0);

        let mut writer = BufWriter::new(Vec::new());
        save_ron(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!("(\n    cookies: [],\n)\n", string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_ron(&cookie_store, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!("(\n    cookies: [],\n)\n", string);

        let mut writer = BufWriter::new(Vec::new());
        save_ron(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!("(\n    cookies: [],\n)\n", string);

        let mut writer = BufWriter::new(Vec::new());
        save_incl_expired_and_nonpersistent_ron(&cookie_store_all, &mut writer).unwrap();
        let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(cookie, string);
    }
}
