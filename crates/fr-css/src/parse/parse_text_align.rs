use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::text_align::TextAlign;

pub fn parse_text_align(parser: &mut Parser<'_>) -> Option<TextAlign> {
    match parse_keyword(parser)?.as_str() {
        "left" | "start" | "justify" => Some(TextAlign::Left),
        "center" => Some(TextAlign::Center),
        "right" | "end" => Some(TextAlign::Right),
        _ => None,
    }
}
