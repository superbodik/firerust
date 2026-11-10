pub const ROOT_FONT_SIZE: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Auto,
}

impl Length {
    pub fn to_font_relative(self, font_size: f32) -> Self {
        match self {
            Self::Em(value) => Self::Px(value * font_size),
            Self::Rem(value) => Self::Px(value * ROOT_FONT_SIZE),
            other => other,
        }
    }

    pub fn px_or_zero(self) -> f32 {
        match self {
            Self::Px(value) => value,
            _ => 0.0,
        }
    }
}
