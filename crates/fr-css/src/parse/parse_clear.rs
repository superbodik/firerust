use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::clear_side::ClearSide;

pub fn parse_clear(parser: &mut Parser<'_>) -> Option<ClearSide> {
    match parse_keyword(parser)?.as_str() {
        "none" => Some(ClearSide::None),
        "left" | "inline-start" => Some(ClearSide::Left),
        "right" | "inline-end" => Some(ClearSide::Right),
        "both" => Some(ClearSide::Both),
        _ => None,
    }
}
