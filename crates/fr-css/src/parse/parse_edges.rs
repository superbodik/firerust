use cssparser::Parser;

use crate::parse::parse_length::parse_length;
use crate::value::edges::Edges;
use crate::value::length::Length;

pub fn parse_edges(parser: &mut Parser<'_>) -> Option<Edges<Length>> {
    let mut values: Vec<Length> = Vec::new();
    while !parser.is_exhausted() {
        values.push(parse_length(parser)?);
        if values.len() > 4 {
            return None;
        }
    }
    match values.as_slice() {
        [all] => Some(Edges::uniform(*all)),
        [vertical, horizontal] => Some(Edges {
            top: *vertical,
            right: *horizontal,
            bottom: *vertical,
            left: *horizontal,
        }),
        [top, horizontal, bottom] => Some(Edges {
            top: *top,
            right: *horizontal,
            bottom: *bottom,
            left: *horizontal,
        }),
        [top, right, bottom, left] => Some(Edges {
            top: *top,
            right: *right,
            bottom: *bottom,
            left: *left,
        }),
        _ => None,
    }
}
