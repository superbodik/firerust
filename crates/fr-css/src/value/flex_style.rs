use crate::value::align_items::AlignItems;
use crate::value::flex_direction::FlexDirection;
use crate::value::justify_content::JustifyContent;
use crate::value::length::Length;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexStyle {
    pub direction: FlexDirection,
    pub is_wrapping: bool,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub row_gap: f32,
    pub column_gap: f32,
    pub grow: f32,
    pub shrink: f32,
    pub basis: Length,
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            is_wrapping: false,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            row_gap: 0.0,
            column_gap: 0.0,
            grow: 0.0,
            shrink: 1.0,
            basis: Length::Auto,
        }
    }
}
