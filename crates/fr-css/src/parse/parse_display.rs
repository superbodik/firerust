use cssparser::Parser;

use crate::value::display::Display;

pub fn parse_display(parser: &mut Parser<'_>) -> Option<Display> {
    let name = parser.expect_ident().ok()?.to_ascii_lowercase();
    match name.as_str() {
        "none" => Some(Display::None),
        "inline" | "inline-block" | "contents" => Some(Display::Inline),
        "block" | "flex" | "grid" | "table" | "flow-root" => Some(Display::Block),
        "list-item" => Some(Display::ListItem),
        _ => None,
    }
}
