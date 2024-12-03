use crate::parser::Parser;
use serde::{Deserialize, Deserializer};

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
            if (!DIGITS && c.is_ascii_digit()) || (!ALPHABETIC && c.is_alphabetic()) {
                return false;
            }
        }

        true
    }
}

impl<const A: usize, const B: bool, const C: bool> Otp<A, B, C> {
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
