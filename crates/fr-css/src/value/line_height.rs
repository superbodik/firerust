pub const NORMAL_LINE_HEIGHT_FACTOR: f32 = 1.4;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LineHeight {
    #[default]
    Normal,
    Number(f32),
    Px(f32),
}

impl LineHeight {
    pub fn resolve(self, font_size: f32) -> f32 {
        match self {
            Self::Normal => font_size * NORMAL_LINE_HEIGHT_FACTOR,
            Self::Number(factor) => font_size * factor,
            Self::Px(value) => value,
        }
    }
}
