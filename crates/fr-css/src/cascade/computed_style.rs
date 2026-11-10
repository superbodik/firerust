use fr_geom::color::rgba::Rgba;

use crate::value::display::Display;
use crate::value::edges::Edges;
use crate::value::length::{Length, ROOT_FONT_SIZE};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedStyle {
    pub display: Display,
    pub color: Rgba,
    pub background_color: Rgba,
    pub font_size: f32,
    pub font_weight: u16,
    pub margin: Edges<Length>,
    pub padding: Edges<Length>,
    pub border_width: Edges<f32>,
    pub border_color: Rgba,
}

impl ComputedStyle {
    pub fn initial() -> Self {
        Self {
            display: Display::Inline,
            color: Rgba::BLACK,
            background_color: Rgba::new(0, 0, 0, 0),
            font_size: ROOT_FONT_SIZE,
            font_weight: 400,
            margin: Edges::uniform(Length::Px(0.0)),
            padding: Edges::uniform(Length::Px(0.0)),
            border_width: Edges::uniform(0.0),
            border_color: Rgba::BLACK,
        }
    }
}
