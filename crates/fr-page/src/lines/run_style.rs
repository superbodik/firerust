use fr_css::cascade::computed_style::ComputedStyle;
use fr_geom::color::rgba::Rgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RunStyle {
    pub color: Rgba,
    pub font_size: f32,
    pub font_weight: u16,
}

impl RunStyle {
    pub fn from_computed(style: &ComputedStyle) -> Self {
        Self {
            color: style.color,
            font_size: style.font_size,
            font_weight: style.font_weight,
        }
    }
}
