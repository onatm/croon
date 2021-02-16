use std::{error, fmt::Display};
#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong!")
    }
}
impl error::Error for Error {}
