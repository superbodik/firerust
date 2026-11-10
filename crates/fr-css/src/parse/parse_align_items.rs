use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::align_items::AlignItems;

pub fn parse_align_items(parser: &mut Parser<'_>) -> Option<AlignItems> {
    match parse_keyword(parser)?.as_str() {
        "stretch" | "normal" => Some(AlignItems::Stretch),
        "flex-start" | "start" | "baseline" | "self-start" => Some(AlignItems::FlexStart),
        "flex-end" | "end" | "self-end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        _ => None,
    }
}
