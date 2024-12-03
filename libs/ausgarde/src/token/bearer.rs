use std::collections::HashMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::token::{DateTimeUtc, TokenSigner};

#[derive(Debug, Serialize, Deserialize)]
pub struct BearerToken {
    // Audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,

    // Expiration time.
    exp: DateTimeUtc,

    // Issued at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<DateTimeUtc>,

    // Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,

    // Not Before
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<DateTimeUtc>,

    // The subject, can be an ID of a User, Service, etc.
    pub sub: String,

    // Extra data, this gets flattened and may overwrite data, be cautious.
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

impl Default for BearerToken {
    fn default() -> Self {
        Self {
            iss: None,
            iat: Some(DateTimeUtc(Utc::now())),
            custom: HashMap::new(),
            nbf: None,
            aud: None,
            sub: String::new(),
            exp: DateTimeUtc(Utc::now() + Duration::minutes(15)),
        }
    }
}

impl TokenSigner for BearerToken {
    fn sign(&self) -> String {
        let secret = std::env::var("AUSGARD_SIGNING_TOKEN").unwrap();

        let key = if secret.starts_with("base64:") {
            EncodingKey::from_base64_secret(&secret[7..]).unwrap()
        } else {
            EncodingKey::from_secret(secret.as_bytes())
        };

        encode(&Header::default(), &self, &key).unwrap()
    }

    fn decode<S>(token: S) -> Option<Self>
    where
        S: AsRef<str>
    {
        let secret = std::env::var("AUSGARD_SIGNING_TOKEN").unwrap();

        let key = if secret.starts_with("base64:") {
            DecodingKey::from_base64_secret(&secret[..7]).unwrap()
        } else {
            DecodingKey::from_secret(secret.as_bytes())
        };

        match decode(token.as_ref(), &key, &Validation::default()) {
            Ok(d) => {
                Some(d.claims)
            }
            Err(_) => None,
        }
    }
}