pub mod email;
pub mod id;
pub mod otp;
pub mod password;
pub mod username;

pub trait Parser {
    fn from_unchecked<S>(data: S) -> Self
    where
        S: AsRef<str>;
    fn valid<S>(data: S) -> bool
    where
        S: AsRef<str>;
}
