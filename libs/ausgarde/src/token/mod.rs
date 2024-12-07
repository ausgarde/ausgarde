use chrono::{TimeZone, Utc};
use jsonwebtoken::Validation;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod jwt;

pub trait TokenGenerator {
    fn encode(&self) -> String;

    fn decode<S>(token: S, validation: Validation) -> Option<Self>
    where
        S: AsRef<str>,
        Self: Sized;
}

#[derive(Debug)]
pub struct DateTimeUtc(pub chrono::DateTime<chrono::Utc>);

impl DateTimeUtc {
    pub fn now() -> Self {
        DateTimeUtc(chrono::Utc::now())
    }

    pub fn now_add_15min() -> Self {
        DateTimeUtc(chrono::Utc::now() + chrono::Duration::minutes(15))
    }

    pub fn now_add_1hour() -> Self {
        DateTimeUtc(chrono::Utc::now() + chrono::Duration::hours(1))
    }
}

impl Serialize for DateTimeUtc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

impl<'de> Deserialize<'de> for DateTimeUtc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;
        Ok(DateTimeUtc(Utc.timestamp_opt(timestamp, 0).unwrap()))
    }
}
