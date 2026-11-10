use cssparser::Parser;
use fr_geom::color::rgba::Rgba;

use crate::parse::parse_color::parse_color;
use crate::parse::try_option::try_option;

pub fn parse_background(parser: &mut Parser<'_>) -> Option<Rgba> {
    let mut found: Option<Rgba> = None;
    while !parser.is_exhausted() {
        if let Some(color) = try_option(parser, parse_color) {
            found = Some(color);
        } else if parser.next().is_err() {
            break;
        }
    }
    found
}
