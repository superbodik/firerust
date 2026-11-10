use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::flex_direction::FlexDirection;

pub fn parse_flex_direction(parser: &mut Parser<'_>) -> Option<FlexDirection> {
    match parse_keyword(parser)?.as_str() {
        "row" => Some(FlexDirection::Row),
        "row-reverse" => Some(FlexDirection::RowReverse),
        "column" => Some(FlexDirection::Column),
        "column-reverse" => Some(FlexDirection::ColumnReverse),
        _ => None,
    }
}
