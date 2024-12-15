use crate::parser::Parser;
use fancy_regex::Regex;
use serde::{Deserialize, Deserializer};

pub const USERNAME_MIN_LENGTH: usize = 2;
pub const USERNAME_MAX_LENGTH: usize = 32;

/// A Username type to parse and validate usernames.
///
/// Usernames must be between 2 and 32 Characters long, and can only contain the following characters:
/// - a-z
/// - A-Z
/// - 0-9
/// - . (dot)
/// - _ (underscore)
/// - - (dash)
///
/// The Parser will not allow consecutive dots, underscores or dashes.
pub struct Username(pub String);

impl Parser for Username {
    fn from_unchecked<S>(data: S) -> Self
    where
        S: AsRef<str>,
    {
        Username(data.as_ref().to_string())
    }

    fn valid<S>(data: S) -> bool
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();
        let len = data.len();

        if !(USERNAME_MIN_LENGTH..=USERNAME_MAX_LENGTH).contains(&len) {
            return false;
        }

        let regex = Regex::new(r"^[a-zA-Z0-9._-](?!.*\.\.)[a-zA-Z0-9._-]*$").unwrap();

        regex.is_match(data.as_ref()).unwrap()
    }
}

impl Username {
    pub fn new<S>(data: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();

        if Username::valid(data) {
            return Some(Username(data.to_string()));
        }
        None
    }
}

impl<'de> Deserialize<'de> for Username {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Username::new(&s).ok_or(serde::de::Error::custom("Invalid username"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        for d in [".hello.world.", "username", "._.fancy._."] {
            assert!(Username::valid(d));
        }
    }

    #[test]
    fn invalid() {
        for d in [".hello..world.", "user name", "a"] {
            assert!(!Username::valid(d));
        }
    }
}
