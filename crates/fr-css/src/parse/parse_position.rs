use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::position::Position;

pub fn parse_position(parser: &mut Parser<'_>) -> Option<Position> {
    match parse_keyword(parser)?.as_str() {
        "static" => Some(Position::Static),
        "relative" => Some(Position::Relative),
        "absolute" => Some(Position::Absolute),
        "fixed" => Some(Position::Fixed),
        "sticky" | "-webkit-sticky" => Some(Position::Sticky),
        _ => None,
    }
}
