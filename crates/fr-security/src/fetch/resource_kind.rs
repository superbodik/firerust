use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Script,
    Style,
    Image,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Script => "script",
            Self::Style => "stylesheet",
            Self::Image => "image",
        };
        formatter.write_str(text)
    }
}
