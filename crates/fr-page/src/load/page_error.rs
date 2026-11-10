use std::error::Error;
use std::fmt;

use fr_net::address::address_error::AddressError;
use fr_net::load::net_error::NetError;

#[derive(Debug)]
pub enum PageError {
    Address(AddressError),
    Load(NetError),
}

impl fmt::Display for PageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(error) => error.fmt(formatter),
            Self::Load(error) => error.fmt(formatter),
        }
    }
}

impl Error for PageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Address(error) => Some(error),
            Self::Load(error) => Some(error),
        }
    }
}

impl From<AddressError> for PageError {
    fn from(error: AddressError) -> Self {
        Self::Address(error)
    }
}

impl From<NetError> for PageError {
    fn from(error: NetError) -> Self {
        Self::Load(error)
    }
}
