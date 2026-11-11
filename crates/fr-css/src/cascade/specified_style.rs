use fr_geom::color::rgba::Rgba;

use crate::parse::declaration::Declaration;
use crate::value::align_items::AlignItems;
use crate::value::box_sizing::BoxSizing;
use crate::value::clear_side::ClearSide;
use crate::value::corners::Corners;
use crate::value::display::Display;
use crate::value::edges::Edges;
use crate::value::flex_direction::FlexDirection;
use crate::value::float_side::FloatSide;
use crate::value::font_family::FontFamily;
use crate::value::font_style::FontStyle;
use crate::value::justify_content::JustifyContent;
use crate::value::length::Length;
use crate::value::line_height::LineHeight;
use crate::value::position::Position;
use crate::value::text_align::TextAlign;
use crate::value::text_decoration::TextDecoration;
use crate::value::white_space::WhiteSpace;

#[derive(Debug, Clone, Default)]
pub struct SpecifiedStyle {
    pub display: Option<Display>,
    pub color: Option<Rgba>,
    pub background_color: Option<Rgba>,
    pub font_size: Option<Length>,
    pub font_weight: Option<u16>,
    pub font_style: Option<FontStyle>,
    pub font_family: Option<FontFamily>,
    pub line_height: Option<LineHeight>,
    pub text_align: Option<TextAlign>,
    pub text_decoration: Option<TextDecoration>,
    pub white_space: Option<WhiteSpace>,
    pub box_sizing: Option<BoxSizing>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub min_width: Option<Length>,
    pub max_width: Option<Length>,
    pub margin: Edges<Option<Length>>,
    pub padding: Edges<Option<Length>>,
    pub border_width: Edges<Option<Length>>,
    pub border_color: Option<Rgba>,
    pub accent_color: Option<Rgba>,
    pub float_side: Option<FloatSide>,
    pub clear_side: Option<ClearSide>,
    pub position: Option<Position>,
    pub border_radius: Corners<Option<Length>>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<bool>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub row_gap: Option<Length>,
    pub column_gap: Option<Length>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<Length>,
}

impl SpecifiedStyle {
    pub fn apply(&mut self, declaration: &Declaration) {
        match declaration {
            Declaration::Color(value) => self.color = Some(*value),
            Declaration::BackgroundColor(value) => self.background_color = Some(*value),
            Declaration::FontSize(value) => self.font_size = Some(*value),
            Declaration::FontWeight(value) => self.font_weight = Some(*value),
            Declaration::FontStyle(value) => self.font_style = Some(*value),
            Declaration::FontFamily(value) => self.font_family = Some(*value),
            Declaration::LineHeight(value) => self.line_height = Some(*value),
            Declaration::TextAlign(value) => self.text_align = Some(*value),
            Declaration::TextDecoration(value) => self.text_decoration = Some(*value),
            Declaration::WhiteSpace(value) => self.white_space = Some(*value),
            Declaration::Display(value) => self.display = Some(*value),
            Declaration::BoxSizing(value) => self.box_sizing = Some(*value),
            Declaration::Width(value) => self.width = Some(*value),
            Declaration::Height(value) => self.height = Some(*value),
            Declaration::MinWidth(value) => self.min_width = Some(*value),
            Declaration::MaxWidth(value) => self.max_width = Some(*value),
            Declaration::Margin(side, value) => self.margin.set(*side, Some(*value)),
            Declaration::Padding(side, value) => self.padding.set(*side, Some(*value)),
            Declaration::BorderWidth(side, value) => self.border_width.set(*side, Some(*value)),
            Declaration::BorderColor(value) => self.border_color = Some(*value),
            Declaration::AccentColor(value) => self.accent_color = Some(*value),
            Declaration::Float(value) => self.float_side = Some(*value),
            Declaration::Clear(value) => self.clear_side = Some(*value),
            Declaration::Position(value) => self.position = Some(*value),
            Declaration::Custom(..) | Declaration::Deferred(..) => {}
            Declaration::BorderRadius(corner, value) => {
                self.border_radius.set(*corner, Some(*value))
            }
            Declaration::FlexDirection(value) => self.flex_direction = Some(*value),
            Declaration::FlexWrap(value) => self.flex_wrap = Some(*value),
            Declaration::JustifyContent(value) => self.justify_content = Some(*value),
            Declaration::AlignItems(value) => self.align_items = Some(*value),
            Declaration::RowGap(value) => self.row_gap = Some(*value),
            Declaration::ColumnGap(value) => self.column_gap = Some(*value),
            Declaration::FlexGrow(value) => self.flex_grow = Some(*value),
            Declaration::FlexShrink(value) => self.flex_shrink = Some(*value),
            Declaration::FlexBasis(value) => self.flex_basis = Some(*value),
        }
    }
}
