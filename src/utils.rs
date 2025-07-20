use std::net::{Ipv4Addr, Ipv6Addr};
use url::Host;
use url::Url;

pub fn is_http_scheme(url: &Url) -> bool {
    url.scheme().starts_with("http")
}

pub fn is_host_name(host: &str) -> bool {
    host.parse::<Ipv4Addr>().is_err() && host.parse::<Ipv6Addr>().is_err()
}

pub fn is_secure(url: &Url) -> bool {
    if url.scheme() == "https" {
        return true;
    }
    if let Some(u) = url.host() {
        match u {
            Host::Domain(d) => d == "localhost",
            Host::Ipv4(ip) => ip.is_loopback(),
            Host::Ipv6(ip) => ip.is_loopback(),
        }
    } else {
        false
    }
}

#[cfg(test)]
pub mod test {
    use crate::cookie::Cookie;
    use time::{Duration, OffsetDateTime};
    use url::Url;
    #[inline]
    pub fn url(url: &str) -> Url {
        Url::parse(url).unwrap()
    }
    #[inline]
    pub fn make_cookie<'a>(
        cookie: &str,
        url_str: &str,
        expires: Option<OffsetDateTime>,
        max_age: Option<u64>,
    ) -> Cookie<'a> {
        Cookie::parse(
            format!(
                "{}{}{}",
                cookie,
                expires.map_or(String::from(""), |e| format!(
                    "; Expires={}",
                    e.format(time::macros::format_description!("[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT")).unwrap()
                )),
                max_age.map_or(String::from(""), |m| format!("; Max-Age={m}"))
            ),
            &url(url_str),
        )
        .unwrap()
    }
    #[inline]
    pub fn in_days(days: i64) -> OffsetDateTime {
        OffsetDateTime::now_utc() + Duration::days(days)
    }
    #[inline]
    pub fn in_minutes(mins: i64) -> OffsetDateTime {
        OffsetDateTime::now_utc() + Duration::minutes(mins)
    }
}
