use fr_geom::color::rgba::Rgba;

use crate::render::text_family::TextFamily;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextRun<'a> {
    pub text: &'a str,
    pub color: Rgba,
    pub font_size: f32,
    pub line_height: f32,
    pub font_weight: u16,
    pub is_italic: bool,
    pub family: TextFamily,
    pub is_underlined: bool,
    pub is_struck: bool,
}
