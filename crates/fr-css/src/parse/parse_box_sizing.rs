use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::box_sizing::BoxSizing;

pub fn parse_box_sizing(parser: &mut Parser<'_>) -> Option<BoxSizing> {
    match parse_keyword(parser)?.as_str() {
        "content-box" => Some(BoxSizing::ContentBox),
        "border-box" => Some(BoxSizing::BorderBox),
        _ => None,
    }
}
