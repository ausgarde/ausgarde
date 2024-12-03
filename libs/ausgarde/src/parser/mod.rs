pub mod password;
pub mod email;
pub mod username;
pub mod otp;

pub trait Parser {
    fn from_unchecked<S>(data: S) -> Self
    where
        S: AsRef<str>;
    fn valid<S>(data: S) -> bool
    where
        S: AsRef<str>;
}
