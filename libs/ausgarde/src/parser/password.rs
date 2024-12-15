use crate::parser::Parser;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Deserializer};

pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 128;

/// A Password type to parse and validate passwords.
///
/// Passwords must be between 8 and 128 Characters long, have at least one Uppercase letter and one Special Character.
/// The Parser, will allow any character, including emojis and whitespace.
pub struct Password(String);

impl Parser for Password {
    fn from_unchecked<S>(data: S) -> Self
    where
        S: AsRef<str>,
    {
        Password(data.as_ref().to_string())
    }

    fn valid<S>(data: S) -> bool
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();
        let len = data.len();

        if !(PASSWORD_MIN_LENGTH..=PASSWORD_MAX_LENGTH).contains(&len) {
            return false;
        }

        let (mut has_uppercase, mut has_special) = (false, false);

        for c in data.chars() {
            if has_special && has_uppercase {
                return true;
            }

            if c.is_whitespace() || c.is_ascii() {
                has_special = true;

                if c.is_uppercase() {
                    has_uppercase = true;
                }
                continue;
            }

            if c.is_uppercase() {
                has_uppercase = true;
                continue;
            }
        }

        has_uppercase && has_special
    }
}

impl Password {
    pub fn new<S>(data: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();
        if Password::valid(data) {
            return Some(Password(String::from(data)));
        }

        None
    }

    /// Consumes `self` and hashes the Password with the Default parameters.
    pub fn to_argon2_hash(self) -> argon2::password_hash::Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        let hasher = Argon2::default();

        Ok(hasher.hash_password(self.0.as_bytes(), &salt)?.to_string())
    }

    pub fn verify_password(&self, hash: &str) -> argon2::password_hash::Result<bool> {
        let hash = PasswordHash::new(hash)?;

        Ok(Argon2::default()
            .verify_password(self.0.as_bytes(), &hash)
            .is_ok())
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = String::deserialize(deserializer)?;

        Password::new(&data).ok_or(serde::de::Error::custom("Invalid password"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        for p in [
            "1Zwei3%padding",
            "Spaces?? 123456789",
            "🦀🦀🦀🦀Crab🦀 123 🦀Crab🦀🦀🦀🦀",
        ] {
            assert!(Password::valid(&p));
        }
    }

    #[test]
    fn invalid() {
        for p in ["invalid:(", "short", "123%no"] {
            assert!(!Password::valid(&p));
        }
    }
}
