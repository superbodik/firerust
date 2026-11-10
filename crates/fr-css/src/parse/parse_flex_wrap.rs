use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;

pub fn parse_flex_wrap(parser: &mut Parser<'_>) -> Option<bool> {
    match parse_keyword(parser)?.as_str() {
        "nowrap" => Some(false),
        "wrap" | "wrap-reverse" => Some(true),
        _ => None,
    }
}
