use fr_geom::color::rgba::Rgba;

use crate::value::align_items::AlignItems;
use crate::value::box_sizing::BoxSizing;
use crate::value::corner::Corner;
use crate::value::display::Display;
use crate::value::flex_direction::FlexDirection;
use crate::value::font_family::FontFamily;
use crate::value::font_style::FontStyle;
use crate::value::justify_content::JustifyContent;
use crate::value::length::Length;
use crate::value::line_height::LineHeight;
use crate::value::side::Side;
use crate::value::text_align::TextAlign;
use crate::value::text_decoration::TextDecoration;
use crate::value::white_space::WhiteSpace;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Color(Rgba),
    BackgroundColor(Rgba),
    FontSize(Length),
    FontWeight(u16),
    FontStyle(FontStyle),
    FontFamily(FontFamily),
    LineHeight(LineHeight),
    TextAlign(TextAlign),
    TextDecoration(TextDecoration),
    WhiteSpace(WhiteSpace),
    Display(Display),
    BoxSizing(BoxSizing),
    Width(Length),
    Height(Length),
    MinWidth(Length),
    MaxWidth(Length),
    Margin(Side, Length),
    Padding(Side, Length),
    BorderWidth(Side, Length),
    BorderColor(Rgba),
    AccentColor(Rgba),
    BorderRadius(Corner, Length),
    Custom(Box<str>, Box<str>),
    Deferred(Box<str>, Box<str>),
    FlexDirection(FlexDirection),
    FlexWrap(bool),
    JustifyContent(JustifyContent),
    AlignItems(AlignItems),
    RowGap(Length),
    ColumnGap(Length),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexBasis(Length),
}
