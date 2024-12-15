use crate::parser::Parser;
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Deserializer};

/// A one-time password generator + parser.
///
/// # Example
/// ```no_run
/// use ausgarde::parser::otp::Otp;
///
/// let otp = Otp::<6, true, true>::generate();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Otp<const LEN: usize, const DIGITS: bool, const ALPHABETIC: bool>(String);

impl<const LEN: usize, const DIGITS: bool, const ALPHABETIC: bool> Parser
    for Otp<LEN, DIGITS, ALPHABETIC>
{
    fn from_unchecked<S>(data: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(String::from(data.as_ref()))
    }

    fn valid<S>(data: S) -> bool
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();

        if data.len() != LEN {
            return false;
        }

        for c in data.chars() {
            if (!DIGITS && c.is_ascii_digit())
                || (!ALPHABETIC && c.is_alphabetic())
                || (!c.is_ascii_digit() && !c.is_alphabetic())
            {
                return false;
            }
        }

        true
    }
}

impl<const LEN: usize, const DIGITS: bool, const ALPHABETIC: bool> Otp<LEN, DIGITS, ALPHABETIC> {
    pub fn new<S>(data: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();

        if Self::valid(data) {
            return Some(Self(data.to_string()));
        }

        None
    }

    pub fn generate() -> Self {
        let mut rng = OsRng;
        let mut otp = String::with_capacity(LEN);

        let mut ciphers = vec![];

        if DIGITS {
            ciphers.extend(b'0'..=b'9');
        }

        if ALPHABETIC {
            ciphers.extend(b'a'..=b'z');
            ciphers.extend(b'A'..=b'Z');
        }

        for _ in 0..LEN {
            let idx = rng.next_u32() % ciphers.len() as u32;
            otp.push(ciphers[idx as usize] as char);
        }

        Self(otp)
    }
}

impl<'de, const A: usize, const B: bool, const C: bool> Deserialize<'de> for Otp<A, B, C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Otp::new(&s).ok_or(serde::de::Error::custom("Invalid Otp"))
    }
}

// Write me some tests using the `Otp` struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(Otp::<6, true, true>::valid("123456"));
        assert!(Otp::<6, true, true>::valid("abcdef"));
        assert!(Otp::<6, true, true>::valid("ABCDEF"));
        assert!(Otp::<6, true, true>::valid("aBcDeF"));
        assert!(!Otp::<6, true, true>::valid("12345"));
        assert!(!Otp::<6, true, true>::valid("1234567"));
        assert!(!Otp::<6, true, true>::valid("12345!"));
        assert!(!Otp::<6, true, true>::valid("12345 "));
        assert!(!Otp::<6, true, true>::valid("12345\n"));
        assert!(!Otp::<6, true, true>::valid("12345\t"));
        assert!(!Otp::<6, true, true>::valid("12345\r"));
    }
}
