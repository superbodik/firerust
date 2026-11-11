use cssparser::Parser;

use crate::parse::declaration::Declaration;
use crate::parse::parse_align_items::parse_align_items;
use crate::parse::parse_background::parse_background;
use crate::parse::parse_border::parse_border;
use crate::parse::parse_border_radius::{parse_border_radius, parse_corner_radius};
use crate::parse::parse_box_sizing::parse_box_sizing;
use crate::parse::parse_clear::parse_clear;
use crate::parse::parse_color::parse_color;
use crate::parse::parse_display::parse_display;
use crate::parse::parse_edges::parse_edges;
use crate::parse::parse_flex::parse_flex;
use crate::parse::parse_flex_direction::parse_flex_direction;
use crate::parse::parse_flex_wrap::parse_flex_wrap;
use crate::parse::parse_float::parse_float;
use crate::parse::parse_font_family::parse_font_family;
use crate::parse::parse_font_style::parse_font_style;
use crate::parse::parse_font_weight::parse_font_weight;
use crate::parse::parse_gap::parse_gap;
use crate::parse::parse_justify_content::parse_justify_content;
use crate::parse::parse_length::parse_length;
use crate::parse::parse_line_height::parse_line_height;
use crate::parse::parse_position::parse_position;
use crate::parse::parse_text_align::parse_text_align;
use crate::parse::parse_text_decoration::parse_text_decoration;
use crate::parse::parse_white_space::parse_white_space;
use crate::parse::side_from_name::side_from_name;
use crate::value::corner::Corner;
use crate::value::edges::Edges;
use crate::value::length::Length;
use crate::value::side::Side;

pub fn parse_property(name: &str, parser: &mut Parser<'_>) -> Option<Vec<Declaration>> {
    match name {
        "color" => single(parse_color(parser), Declaration::Color),
        "background-color" => single(parse_color(parser), Declaration::BackgroundColor),
        "background" => single(parse_background(parser), Declaration::BackgroundColor),
        "font-size" => single(parse_length(parser), Declaration::FontSize),
        "font-weight" => single(parse_font_weight(parser), Declaration::FontWeight),
        "font-style" => single(parse_font_style(parser), Declaration::FontStyle),
        "font-family" => single(parse_font_family(parser), Declaration::FontFamily),
        "line-height" => single(parse_line_height(parser), Declaration::LineHeight),
        "text-align" => single(parse_text_align(parser), Declaration::TextAlign),
        "text-decoration" | "text-decoration-line" => {
            single(parse_text_decoration(parser), Declaration::TextDecoration)
        }
        "white-space" => single(parse_white_space(parser), Declaration::WhiteSpace),
        "display" => single(parse_display(parser), Declaration::Display),
        "box-sizing" => single(parse_box_sizing(parser), Declaration::BoxSizing),
        "width" => single(parse_length(parser), Declaration::Width),
        "height" => single(parse_length(parser), Declaration::Height),
        "min-width" => single(parse_length(parser), Declaration::MinWidth),
        "max-width" => single(parse_max_width(parser), Declaration::MaxWidth),
        "margin" => parse_edges(parser).map(|edges| expand(edges, Declaration::Margin)),
        "padding" => parse_edges(parser).map(|edges| expand(edges, Declaration::Padding)),
        "border-width" => parse_edges(parser).map(|edges| expand(edges, Declaration::BorderWidth)),
        "border-color" => single(parse_color(parser), Declaration::BorderColor),
        "accent-color" => single(parse_color(parser), Declaration::AccentColor),
        "float" => single(parse_float(parser), Declaration::Float),
        "clear" => single(parse_clear(parser), Declaration::Clear),
        "position" => single(parse_position(parser), Declaration::Position),
        "border" => parse_border(parser, &Side::ALL),
        "border-radius" => parse_border_radius(parser),
        "border-top-left-radius" => parse_corner_radius(parser, Corner::TopLeft),
        "border-top-right-radius" => parse_corner_radius(parser, Corner::TopRight),
        "border-bottom-right-radius" => parse_corner_radius(parser, Corner::BottomRight),
        "border-bottom-left-radius" => parse_corner_radius(parser, Corner::BottomLeft),
        "flex-direction" => single(parse_flex_direction(parser), Declaration::FlexDirection),
        "flex-wrap" => single(parse_flex_wrap(parser), Declaration::FlexWrap),
        "justify-content" => single(parse_justify_content(parser), Declaration::JustifyContent),
        "align-items" => single(parse_align_items(parser), Declaration::AlignItems),
        "gap" | "grid-gap" => parse_gap(parser),
        "row-gap" => single(parse_length(parser), Declaration::RowGap),
        "column-gap" => single(parse_length(parser), Declaration::ColumnGap),
        "flex" => parse_flex(parser),
        "flex-grow" => single(parse_number(parser), Declaration::FlexGrow),
        "flex-shrink" => single(parse_number(parser), Declaration::FlexShrink),
        "flex-basis" => single(parse_length(parser), Declaration::FlexBasis),
        _ => parse_sided(name, parser),
    }
}

fn single<T>(value: Option<T>, make: fn(T) -> Declaration) -> Option<Vec<Declaration>> {
    value.map(|inner| vec![make(inner)])
}

fn parse_max_width(parser: &mut Parser<'_>) -> Option<Length> {
    let keyword = parser.try_parse(|inner| inner.expect_ident_matching("none"));
    if keyword.is_ok() {
        return Some(Length::Auto);
    }
    parse_length(parser)
}

fn parse_number(parser: &mut Parser<'_>) -> Option<f32> {
    let value = parser.expect_number().ok()?;
    (value >= 0.0).then_some(value)
}

fn expand(edges: Edges<Length>, make: fn(Side, Length) -> Declaration) -> Vec<Declaration> {
    Side::ALL
        .iter()
        .map(|&side| make(side, edges.get(side)))
        .collect()
}

fn parse_sided(name: &str, parser: &mut Parser<'_>) -> Option<Vec<Declaration>> {
    if let Some(side) = name.strip_prefix("margin-").and_then(side_from_name) {
        return parse_length(parser).map(|value| vec![Declaration::Margin(side, value)]);
    }
    if let Some(side) = name.strip_prefix("padding-").and_then(side_from_name) {
        return parse_length(parser).map(|value| vec![Declaration::Padding(side, value)]);
    }
    let rest = name.strip_prefix("border-")?;
    if let Some(side) = side_from_name(rest) {
        return parse_border(parser, &[side]);
    }
    let side = rest.strip_suffix("-width").and_then(side_from_name)?;
    parse_length(parser).map(|value| vec![Declaration::BorderWidth(side, value)])
}
