use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::display::Display;

pub fn parse_display(parser: &mut Parser<'_>) -> Option<Display> {
    match parse_keyword(parser)?.as_str() {
        "none" => Some(Display::None),
        "inline" | "inline-block" | "contents" => Some(Display::Inline),
        "block" | "grid" | "inline-grid" | "flow-root" | "table-caption" => Some(Display::Block),
        "flex" | "inline-flex" => Some(Display::Flex),
        "list-item" => Some(Display::ListItem),
        "table" | "inline-table" => Some(Display::Table),
        "table-row-group" | "table-header-group" | "table-footer-group" => {
            Some(Display::TableRowGroup)
        }
        "table-row" => Some(Display::TableRow),
        "table-cell" => Some(Display::TableCell),
        _ => None,
    }
}
