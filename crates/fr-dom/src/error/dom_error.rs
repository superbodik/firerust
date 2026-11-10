use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomError {
    UnknownNode,
    AlreadyAttached,
    WouldCreateCycle,
    TooManyNodes,
}

impl fmt::Display for DomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::UnknownNode => "node does not belong to this document",
            Self::AlreadyAttached => "node already has a parent",
            Self::WouldCreateCycle => "node cannot become its own ancestor",
            Self::TooManyNodes => "document holds too many nodes",
        };
        formatter.write_str(text)
    }
}

impl Error for DomError {}
