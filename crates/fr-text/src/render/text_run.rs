use fr_geom::color::rgba::Rgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextRun<'a> {
    pub text: &'a str,
    pub color: Rgba,
    pub font_size: f32,
    pub font_weight: u16,
}
