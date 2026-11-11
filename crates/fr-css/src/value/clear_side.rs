#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClearSide {
    #[default]
    None,
    Left,
    Right,
    Both,
}

impl ClearSide {
    pub fn clears_left(self) -> bool {
        matches!(self, Self::Left | Self::Both)
    }

    pub fn clears_right(self) -> bool {
        matches!(self, Self::Right | Self::Both)
    }
}
