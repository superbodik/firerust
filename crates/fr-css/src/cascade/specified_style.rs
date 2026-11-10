use fr_geom::color::rgba::Rgba;

use crate::parse::declaration::Declaration;
use crate::value::display::Display;
use crate::value::edges::Edges;
use crate::value::length::Length;

#[derive(Debug, Clone, Default)]
pub struct SpecifiedStyle {
    pub display: Option<Display>,
    pub color: Option<Rgba>,
    pub background_color: Option<Rgba>,
    pub font_size: Option<Length>,
    pub font_weight: Option<u16>,
    pub margin: Edges<Option<Length>>,
    pub padding: Edges<Option<Length>>,
    pub border_width: Edges<Option<Length>>,
    pub border_color: Option<Rgba>,
}

impl SpecifiedStyle {
    pub fn apply(&mut self, declaration: &Declaration) {
        match declaration {
            Declaration::Color(value) => self.color = Some(*value),
            Declaration::BackgroundColor(value) => self.background_color = Some(*value),
            Declaration::FontSize(value) => self.font_size = Some(*value),
            Declaration::FontWeight(value) => self.font_weight = Some(*value),
            Declaration::Display(value) => self.display = Some(*value),
            Declaration::Margin(side, value) => self.margin.set(*side, Some(*value)),
            Declaration::Padding(side, value) => self.padding.set(*side, Some(*value)),
            Declaration::BorderWidth(side, value) => self.border_width.set(*side, Some(*value)),
            Declaration::BorderColor(value) => self.border_color = Some(*value),
        }
    }
}
