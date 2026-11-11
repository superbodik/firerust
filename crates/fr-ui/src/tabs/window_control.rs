#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowControl {
    Minimize,
    Maximize,
    Close,
}

impl WindowControl {
    pub const ALL: [WindowControl; 3] = [Self::Minimize, Self::Maximize, Self::Close];

    pub const fn slot(self) -> f32 {
        match self {
            Self::Minimize => 0.0,
            Self::Maximize => 1.0,
            Self::Close => 2.0,
        }
    }
}
