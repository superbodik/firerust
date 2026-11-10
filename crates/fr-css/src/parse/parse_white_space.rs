use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::white_space::WhiteSpace;

pub fn parse_white_space(parser: &mut Parser<'_>) -> Option<WhiteSpace> {
    match parse_keyword(parser)?.as_str() {
        "normal" | "nowrap" => Some(WhiteSpace::Normal),
        "pre" | "pre-wrap" | "pre-line" | "break-spaces" => Some(WhiteSpace::Pre),
        _ => None,
    }
}
