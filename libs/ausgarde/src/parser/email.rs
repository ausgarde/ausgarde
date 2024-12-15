use crate::parser::Parser;
use fancy_regex::Regex;
use serde::{Deserialize, Deserializer};

/// An Email parser used parse and validate email addresses.
///
/// This parser is intended to be used with `serde`.
///
/// # Example
/// ```no_run
/// use ausgarde::parser::Email;
///
/// #[derive(serde::Deserialize)]
/// struct User {
///    email: Email,
/// }
/// ```
pub struct Email(pub String);

impl Parser for Email {
    fn from_unchecked<S>(data: S) -> Self
    where
        S: AsRef<str>,
    {
        Email(data.as_ref().to_string())
    }

    fn valid<S>(data: S) -> bool
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();
        let regex = Regex::new(r"^([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22))*\x40([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d))*$").unwrap();

        regex.is_match(data).unwrap()
    }
}

impl Email {
    pub fn new<S>(email: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let email = email.as_ref();

        if Self::valid(email) {
            return Some(Email(email.to_string()));
        }

        None
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Email::new(&s).ok_or(serde::de::Error::custom("Invalid email"))
    }
}
