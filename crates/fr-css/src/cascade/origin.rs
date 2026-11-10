#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    UserAgent,
    Author,
}

impl Origin {
    pub fn rank(self) -> u8 {
        match self {
            Self::UserAgent => 0,
            Self::Author => 1,
        }
    }
}
