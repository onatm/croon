use std::{error, fmt::Display, str::FromStr};

use crate::parser::Parser;
#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong!")
    }
}
impl error::Error for Error {}

#[derive(Debug)]
pub struct Schedule;

impl FromStr for Schedule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Parser::parse(s))
    }
}
