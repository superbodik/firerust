use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::justify_content::JustifyContent;

pub fn parse_justify_content(parser: &mut Parser<'_>) -> Option<JustifyContent> {
    match parse_keyword(parser)?.as_str() {
        "flex-start" | "start" | "left" | "normal" => Some(JustifyContent::FlexStart),
        "flex-end" | "end" | "right" => Some(JustifyContent::FlexEnd),
        "center" => Some(JustifyContent::Center),
        "space-between" => Some(JustifyContent::SpaceBetween),
        "space-around" => Some(JustifyContent::SpaceAround),
        "space-evenly" => Some(JustifyContent::SpaceEvenly),
        _ => None,
    }
}
