use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
pub struct SerializableTm(OffsetDateTime);

impl std::ops::Deref for SerializableTm {
    type Target = OffsetDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OffsetDateTime> for SerializableTm {
    fn from(tm: OffsetDateTime) -> SerializableTm {
        SerializableTm(tm)
    }
}

/// When a given `Cookie` expires
#[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CookieExpiration {
    /// `Cookie` expires at the given UTC time, as set from either the Max-Age
    /// or Expires attribute of a Set-Cookie header
    AtUtc(SerializableTm),
    /// `Cookie` expires at the end of the current `Session`; this means the cookie
    /// is not persistent
    SessionEnd,
}

impl CookieExpiration {
    /// Indicates if the `Cookie` is expired as of *now*.
    pub fn is_expired(&self) -> bool {
        self.expires_by(&OffsetDateTime::now())
    }

    /// Indicates if the `Cookie` expires as of `utc_tm`.
    pub fn expires_by(&self, utc_tm: &OffsetDateTime) -> bool {
        match *self {
            CookieExpiration::AtUtc(ref expire_tm) => &expire_tm.0 <= utc_tm,
            CookieExpiration::SessionEnd => false,
        }
    }
}

impl From<u64> for CookieExpiration {
    fn from(max_age: u64) -> CookieExpiration {
        // If delta-seconds is less than or equal to zero (0), let expiry-time
        //    be the earliest representable date and time.  Otherwise, let the
        //    expiry-time be the current date and time plus delta-seconds seconds.
        let utc_tm = if 0 == max_age {
            OffsetDateTime::unix_epoch()
        } else {
            // make sure we don't trigger a panic! in Duration by restricting the seconds
            // to the max
            let max_age = std::cmp::min(Duration::max_value().whole_seconds() as u64, max_age);
            let utc_tm = OffsetDateTime::now() + Duration::seconds(max_age as i64);
            utc_tm
        };
        CookieExpiration::from(utc_tm)
    }
}

impl From<OffsetDateTime> for CookieExpiration {
    fn from(utc_tm: OffsetDateTime) -> CookieExpiration {
        CookieExpiration::AtUtc(SerializableTm::from(utc_tm))
    }
}

impl From<Duration> for CookieExpiration {
    fn from(duration: Duration) -> Self {
        // If delta-seconds is less than or equal to zero (0), let expiry-time
        //    be the earliest representable date and time.  Otherwise, let the
        //    expiry-time be the current date and time plus delta-seconds seconds.
        let utc_tm = if duration.is_zero() {
            OffsetDateTime::unix_epoch()
        } else {
            OffsetDateTime::now() + duration
        };
        CookieExpiration::from(utc_tm)
    }
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::CookieExpiration;
    use crate::utils::test::*;

    #[test]
    fn max_age_bounds() {
        match CookieExpiration::from(Duration::max_value().whole_seconds() as u64 + 1) {
            CookieExpiration::AtUtc(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn expired() {
        let ma = CookieExpiration::from(0u64); // Max-Age<=0 indicates the cookie is expired
        assert!(ma.is_expired());
        assert!(ma.expires_by(&in_days(-1)));
    }

    #[test]
    fn max_age() {
        let ma = CookieExpiration::from(60u64);
        assert!(!ma.is_expired());
        assert!(ma.expires_by(&in_minutes(2)));
    }

    #[test]
    fn session_end() {
        // SessionEnd never "expires"; lives until end of session
        let se = CookieExpiration::SessionEnd;
        assert!(!se.is_expired());
        assert!(!se.expires_by(&in_days(1)));
        assert!(!se.expires_by(&in_days(-1)));
    }

    #[test]
    fn at_utc() {
        {
            let expire_tmrw = CookieExpiration::from(in_days(1));
            assert!(!expire_tmrw.is_expired());
            assert!(expire_tmrw.expires_by(&in_days(2)));
        }
        {
            let expired_yest = CookieExpiration::from(in_days(-1));
            assert!(expired_yest.is_expired());
            assert!(!expired_yest.expires_by(&in_days(-2)));
        }
    }
}

mod serde_serialization {
    use super::SerializableTm;
    use serde::de;
    use std::fmt;
    use time;
    use time::OffsetDateTime;

    impl serde::Serialize for SerializableTm {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&format!("{}", self.0.format("%Y-%m-%dT%H:%M:%S%z")))
        }
    }

    impl<'a> serde::Deserialize<'a> for SerializableTm {
        fn deserialize<D>(deserializer: D) -> Result<SerializableTm, D::Error>
        where
            D: de::Deserializer<'a>,
        {
            deserializer.deserialize_str(TmVisitor)
        }
    }

    struct TmVisitor;

    impl<'a> de::Visitor<'a> for TmVisitor {
        type Value = SerializableTm;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("datetime")
        }

        fn visit_str<E>(self, str_data: &str) -> Result<SerializableTm, E>
        where
            E: de::Error,
        {
            OffsetDateTime::parse(str_data, "%Y-%m-%dT%H:%M:%S%z")
                .map(SerializableTm::from)
                .map_err(|_| {
                    E::custom(format!(
                        "could not parse '{}' as a UTC time in RFC3339 format",
                        str_data
                    ))
                })
        }
    }

    #[cfg(test)]
    mod tests {
        use time::OffsetDateTime;

        use crate::cookie_expiration::CookieExpiration;

        fn encode_decode(ce: &CookieExpiration, exp_json: &str) {
            let encoded = serde_json::to_string(ce).unwrap();
            assert_eq!(exp_json, encoded);
            let decoded: CookieExpiration = serde_json::from_str(&encoded).unwrap();
            assert_eq!(*ce, decoded);
        }

        #[test]
        fn serde() {
            let at_utc =
                OffsetDateTime::parse("2015-08-11T16:41:42+0000", "%Y-%m-%dT%H:%M:%S%z").unwrap();
            encode_decode(
                &CookieExpiration::from(at_utc),
                "{\"AtUtc\":\"2015-08-11T16:41:42+0000\"}",
            );
            encode_decode(&CookieExpiration::SessionEnd, "\"SessionEnd\"");
        }
    }
}
