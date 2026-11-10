use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockReason {
    UnsupportedScheme,
    MixedContent,
    LocalFileFromWeb,
    ContentPolicy,
}

impl fmt::Display for BlockReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::UnsupportedScheme => "unsupported address scheme",
            Self::MixedContent => "insecure content on a secure page",
            Self::LocalFileFromWeb => "local file requested by a web page",
            Self::ContentPolicy => "content security policy",
        };
        formatter.write_str(text)
    }
}
