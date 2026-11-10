use crate::value::length::Length;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizeStyle {
    pub width: Length,
    pub height: Length,
    pub min_width: Length,
    pub max_width: Length,
}

impl Default for SizeStyle {
    fn default() -> Self {
        Self {
            width: Length::Auto,
            height: Length::Auto,
            min_width: Length::Auto,
            max_width: Length::Auto,
        }
    }
}
