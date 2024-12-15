use crate::token::{DateTimeUtc, TokenGenerator};
pub use jsonwebtoken;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct JwtBuilder {
    pub iss: Option<Vec<String>>, // Issuer
    pub sub: Option<String>,      // Subject
    pub aud: Option<String>,      // Audience
    pub exp: Option<DateTimeUtc>, // Expiration time
    pub nbf: Option<DateTimeUtc>, // Not before
    pub iat: Option<DateTimeUtc>, // Issued at
    pub jti: Option<String>,      // JWT ID

    // Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl JwtBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the issuer of the token.
    pub fn iss(&mut self, value: &[&str]) -> &mut Self {
        self.iss = Some(value.iter().map(|x| x.to_string()).collect());
        self
    }

    /// Set the subject of the token.
    ///
    /// Most often, this is a id of something, like a user.
    pub fn sub(&mut self, value: impl Into<String>) -> &mut Self {
        self.sub = Some(value.into());
        self
    }

    /// Set the audience of the token.
    pub fn aud(&mut self, value: &[&str]) -> &mut Self {
        self.aud = Some(value.iter().map(|x| x.to_string()).collect());
        self
    }

    /// Set the expiration time of the token.
    pub fn exp(&mut self, value: DateTimeUtc) -> &mut Self {
        self.exp = Some(value);
        self
    }

    /// Set the not before time of the token.
    pub fn nbf(&mut self, value: DateTimeUtc) -> &mut Self {
        self.nbf = Some(value);
        self
    }

    /// Set the issued at time of the token.
    pub fn iat(&mut self, value: DateTimeUtc) -> &mut Self {
        self.iat = Some(value);
        self
    }

    /// Set the JWT ID of the token.
    pub fn jti(&mut self, value: impl Into<String>) -> &mut Self {
        self.jti = Some(value.into());
        self
    }

    /// Add a custom field to the token.
    ///
    /// This can be used to add any custom field to the token.
    /// But be aware that the field name should not conflict with the standard fields,
    /// as it will be overwritten.
    pub fn add_custom<K, V>(&mut self, field: (K, V)) -> &mut Self
    where
        K: Into<String>,
        V: Into<serde_json::Value>,
    {
        self.custom.insert(field.0.into(), field.1.into());
        self
    }
}

impl TokenGenerator for JwtBuilder {
    fn encode(&self) -> String {
        let secret = std::env::var("AUSGARDE_SIGNING_TOKEN").unwrap();

        let key = if secret.starts_with("base64:") {
            EncodingKey::from_base64_secret(secret.strip_prefix("base64:").unwrap()).unwrap()
        } else {
            EncodingKey::from_secret(secret.as_bytes())
        };

        encode(&Header::default(), &self, &key).unwrap()
    }

    fn decode<S>(token: S, validation: Validation) -> Option<Self>
    where
        S: AsRef<str>,
        Self: Sized,
    {
        let secret = std::env::var("AUSGARDE_SIGNING_TOKEN").unwrap();

        let key = if secret.starts_with("base64:") {
            DecodingKey::from_base64_secret(secret.strip_prefix("base64:").unwrap()).unwrap()
        } else {
            DecodingKey::from_secret(secret.as_bytes())
        };

        match decode(token.as_ref(), &key, &validation) {
            Ok(d) => Some(d.claims),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        std::env::set_var("AUSGARDE_SIGNING_TOKEN", "token");

        let token = JwtBuilder::new()
            .iss(&["ausgarde"])
            .sub("test")
            .aud(&["ausgarde"])
            .exp(DateTimeUtc::now_add_15min())
            .iat(DateTimeUtc::now())
            .jti("test")
            .add_custom(("test", "test"))
            .encode();

        let mut val = Validation::default();
        val.set_audience(&["ausgarde"]);
        val.set_issuer(&["ausgarde"]);

        let decoded = JwtBuilder::decode(token, val).unwrap();

        assert_eq!(decoded.iss, Some(vec!["ausgarde".to_string()]));
        assert_eq!(decoded.sub, Some("test".to_string()));
        assert_eq!(decoded.aud, Some("ausgarde".to_string()));
        assert_eq!(decoded.jti, Some("test".to_string()));
        assert_eq!(
            decoded.custom.get("test"),
            Some(&serde_json::Value::String("test".to_string()))
        );
    }
}
