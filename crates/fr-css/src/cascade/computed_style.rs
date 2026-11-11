use fr_geom::color::rgba::Rgba;

use crate::value::box_sizing::BoxSizing;
use crate::value::clear_side::ClearSide;
use crate::value::corners::Corners;
use crate::value::display::Display;
use crate::value::edges::Edges;
use crate::value::flex_style::FlexStyle;
use crate::value::float_side::FloatSide;
use crate::value::font_family::FontFamily;
use crate::value::font_style::FontStyle;
use crate::value::length::{Length, ROOT_FONT_SIZE};
use crate::value::line_height::LineHeight;
use crate::value::size_style::SizeStyle;
use crate::value::text_align::TextAlign;
use crate::value::text_decoration::TextDecoration;
use crate::value::white_space::WhiteSpace;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedStyle {
    pub display: Display,
    pub color: Rgba,
    pub background_color: Rgba,
    pub font_size: f32,
    pub font_weight: u16,
    pub font_style: FontStyle,
    pub font_family: FontFamily,
    pub line_height: LineHeight,
    pub text_align: TextAlign,
    pub text_decoration: TextDecoration,
    pub white_space: WhiteSpace,
    pub box_sizing: BoxSizing,
    pub size: SizeStyle,
    pub margin: Edges<Length>,
    pub padding: Edges<Length>,
    pub border_width: Edges<f32>,
    pub border_color: Rgba,
    pub accent_color: Option<Rgba>,
    pub float_side: FloatSide,
    pub clear_side: ClearSide,
    pub border_radius: Corners<Length>,
    pub flex: FlexStyle,
}

impl ComputedStyle {
    pub fn initial() -> Self {
        Self {
            display: Display::Inline,
            color: Rgba::BLACK,
            background_color: Rgba::new(0, 0, 0, 0),
            font_size: ROOT_FONT_SIZE,
            font_weight: 400,
            font_style: FontStyle::Normal,
            font_family: FontFamily::SansSerif,
            line_height: LineHeight::Normal,
            text_align: TextAlign::Left,
            text_decoration: TextDecoration::NONE,
            white_space: WhiteSpace::Normal,
            box_sizing: BoxSizing::ContentBox,
            size: SizeStyle::default(),
            margin: Edges::uniform(Length::Px(0.0)),
            padding: Edges::uniform(Length::Px(0.0)),
            border_width: Edges::uniform(0.0),
            border_color: Rgba::BLACK,
            accent_color: None,
            float_side: FloatSide::None,
            clear_side: ClearSide::None,
            border_radius: Corners::uniform(Length::Px(0.0)),
            flex: FlexStyle::default(),
        }
    }
}
