use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    Empty,
    Invalid(String),
    UnsupportedScheme(String),
}

impl fmt::Display for AddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("address is empty"),
            Self::Invalid(input) => write!(formatter, "address is not valid: {input}"),
            Self::UnsupportedScheme(scheme) => {
                write!(formatter, "address scheme is not supported: {scheme}")
            }
        }
    }
}

impl Error for AddressError {}
