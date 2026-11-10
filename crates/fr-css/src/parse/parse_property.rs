use cssparser::Parser;

use crate::parse::declaration::Declaration;
use crate::parse::parse_background::parse_background;
use crate::parse::parse_border::parse_border;
use crate::parse::parse_color::parse_color;
use crate::parse::parse_display::parse_display;
use crate::parse::parse_edges::parse_edges;
use crate::parse::parse_font_weight::parse_font_weight;
use crate::parse::parse_length::parse_length;
use crate::parse::side_from_name::side_from_name;
use crate::value::edges::Edges;
use crate::value::length::Length;
use crate::value::side::Side;

pub fn parse_property(name: &str, parser: &mut Parser<'_>) -> Option<Vec<Declaration>> {
    match name {
        "color" => parse_color(parser).map(|value| vec![Declaration::Color(value)]),
        "background-color" => {
            parse_color(parser).map(|value| vec![Declaration::BackgroundColor(value)])
        }
        "background" => {
            parse_background(parser).map(|value| vec![Declaration::BackgroundColor(value)])
        }
        "font-size" => parse_length(parser).map(|value| vec![Declaration::FontSize(value)]),
        "font-weight" => {
            parse_font_weight(parser).map(|value| vec![Declaration::FontWeight(value)])
        }
        "display" => parse_display(parser).map(|value| vec![Declaration::Display(value)]),
        "margin" => parse_edges(parser).map(|edges| expand(edges, Declaration::Margin)),
        "padding" => parse_edges(parser).map(|edges| expand(edges, Declaration::Padding)),
        "border-width" => parse_edges(parser).map(|edges| expand(edges, Declaration::BorderWidth)),
        "border-color" => parse_color(parser).map(|value| vec![Declaration::BorderColor(value)]),
        "border" => parse_border(parser, &Side::ALL),
        _ => parse_sided(name, parser),
    }
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
