use cssparser::Parser;
use fr_geom::color::rgba::Rgba;

use crate::parse::declaration::Declaration;
use crate::parse::parse_color::parse_color;
use crate::parse::parse_length::parse_length;
use crate::parse::try_option::try_option;
use crate::value::length::Length;
use crate::value::side::Side;

pub fn parse_border(parser: &mut Parser<'_>, sides: &[Side]) -> Option<Vec<Declaration>> {
    let mut width: Option<Length> = None;
    let mut color: Option<Rgba> = None;
    let mut has_style = false;
    let mut is_hidden = false;

    while !parser.is_exhausted() {
        if let Some(value) = try_option(parser, parse_length) {
            width = Some(value);
            continue;
        }
        if let Some(value) = try_option(parser, parse_color) {
            color = Some(value);
            continue;
        }
        let keyword = parser.expect_ident().ok()?.to_ascii_lowercase();
        match keyword.as_str() {
            "none" | "hidden" => is_hidden = true,
            "solid" | "dashed" | "dotted" | "double" | "groove" | "ridge" | "inset" | "outset" => {
                has_style = true
            }
            "thin" => width = Some(Length::Px(1.0)),
            "medium" => width = Some(Length::Px(3.0)),
            "thick" => width = Some(Length::Px(5.0)),
            _ => return None,
        }
    }

    let width_value = if is_hidden || !has_style {
        Length::Px(0.0)
    } else {
        width.unwrap_or(Length::Px(3.0))
    };
    let mut declarations: Vec<Declaration> = sides
        .iter()
        .map(|&side| Declaration::BorderWidth(side, width_value))
        .collect();
    if let Some(value) = color {
        declarations.push(Declaration::BorderColor(value));
    }
    Some(declarations)
}
