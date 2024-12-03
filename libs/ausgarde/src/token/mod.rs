use chrono::{TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod bearer;

pub trait TokenSigner {
    fn sign(&self) -> String;
    fn decode<S>(token: S) -> Option<Self>
        where
            S: AsRef<str>,
            Self: Sized;
}

#[derive(Debug)]
pub struct DateTimeUtc(pub chrono::DateTime<chrono::Utc>);

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