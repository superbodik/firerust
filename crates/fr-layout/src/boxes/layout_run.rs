use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::font_family::FontFamily;
use fr_css::value::font_style::FontStyle;
use fr_dom::node::node_id::NodeId;
use fr_geom::color::rgba::Rgba;
use fr_text::render::text_family::TextFamily;
use fr_text::render::text_run::TextRun;

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutRun {
    pub text: String,
    pub color: Rgba,
    pub font_size: f32,
    pub line_height: f32,
    pub font_weight: u16,
    pub is_italic: bool,
    pub family: TextFamily,
    pub link: Option<NodeId>,
}

impl LayoutRun {
    pub fn from_style(text: String, style: &ComputedStyle) -> Self {
        Self {
            text,
            color: style.color,
            font_size: style.font_size,
            line_height: style.line_height.resolve(style.font_size),
            font_weight: style.font_weight,
            is_italic: style.font_style == FontStyle::Italic,
            family: match style.font_family {
                FontFamily::SansSerif => TextFamily::SansSerif,
                FontFamily::Serif => TextFamily::Serif,
                FontFamily::Monospace => TextFamily::Monospace,
            },
            link: None,
        }
    }

    pub fn with_link(mut self, link: Option<NodeId>) -> Self {
        self.link = link;
        self
    }

    pub fn has_same_style(&self, other: &Self) -> bool {
        self.color == other.color
            && self.font_size == other.font_size
            && self.line_height == other.line_height
            && self.font_weight == other.font_weight
            && self.is_italic == other.is_italic
            && self.family == other.family
            && self.link == other.link
    }

    pub fn as_text_run(&self) -> TextRun<'_> {
        TextRun {
            text: &self.text,
            color: self.color,
            font_size: self.font_size,
            line_height: self.line_height,
            font_weight: self.font_weight,
            is_italic: self.is_italic,
            family: self.family,
        }
    }
}
