use cssparser::{Parser, Token};

use crate::parse::declaration::Declaration;
use crate::parse::parse_length::parse_length;
use crate::parse::try_option::try_option;
use crate::value::length::Length;

pub fn parse_flex(parser: &mut Parser<'_>) -> Option<Vec<Declaration>> {
    if let Some(keyword) = try_option(parser, parse_flex_keyword) {
        return Some(declarations(keyword.0, keyword.1, keyword.2));
    }
    let mut grow: Option<f32> = None;
    let mut shrink: Option<f32> = None;
    let mut basis: Option<Length> = None;
    while !parser.is_exhausted() {
        if let Some(number) = try_option(parser, parse_number) {
            if grow.is_none() {
                grow = Some(number);
            } else if shrink.is_none() {
                shrink = Some(number);
            } else {
                basis = Some(Length::Px(number));
            }
        } else {
            basis = Some(parse_length(parser)?);
        }
    }
    grow?;
    let basis_value = basis.unwrap_or(Length::Px(0.0));
    Some(declarations(grow?, shrink.unwrap_or(1.0), basis_value))
}

fn parse_number(parser: &mut Parser<'_>) -> Option<f32> {
    match parser.next().ok()?.clone() {
        Token::Number { value, .. } if value >= 0.0 => Some(value),
        _ => None,
    }
}

fn parse_flex_keyword(parser: &mut Parser<'_>) -> Option<(f32, f32, Length)> {
    let name = parser.expect_ident().ok()?.to_ascii_lowercase();
    match name.as_str() {
        "none" => Some((0.0, 0.0, Length::Auto)),
        "auto" => Some((1.0, 1.0, Length::Auto)),
        "initial" => Some((0.0, 1.0, Length::Auto)),
        _ => None,
    }
}

fn declarations(grow: f32, shrink: f32, basis: Length) -> Vec<Declaration> {
    vec![
        Declaration::FlexGrow(grow),
        Declaration::FlexShrink(shrink),
        Declaration::FlexBasis(basis),
    ]
}
