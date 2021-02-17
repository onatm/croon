use std::{error, fmt::Display};

// `Error` type is created to enable usage of `FromStr` trait on `CronTab`.
#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong!")
    }
}
impl error::Error for Error {}
