use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::float_side::FloatSide;

pub fn parse_float(parser: &mut Parser<'_>) -> Option<FloatSide> {
    match parse_keyword(parser)?.as_str() {
        "none" => Some(FloatSide::None),
        "left" | "inline-start" => Some(FloatSide::Left),
        "right" | "inline-end" => Some(FloatSide::Right),
        _ => None,
    }
}
