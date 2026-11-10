use cssparser::Parser;

use crate::parse::declaration::Declaration;
use crate::parse::parse_length::parse_length;
use crate::value::length::Length;

pub fn parse_gap(parser: &mut Parser<'_>) -> Option<Vec<Declaration>> {
    let row: Length = parse_length(parser)?;
    let column: Length = if parser.is_exhausted() {
        row
    } else {
        parse_length(parser)?
    };
    Some(vec![
        Declaration::RowGap(row),
        Declaration::ColumnGap(column),
    ])
}
