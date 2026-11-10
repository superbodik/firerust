use cssparser::Parser;

use crate::parse::parse_keyword::parse_keyword;
use crate::value::font_style::FontStyle;

pub fn parse_font_style(parser: &mut Parser<'_>) -> Option<FontStyle> {
    match parse_keyword(parser)?.as_str() {
        "normal" => Some(FontStyle::Normal),
        "italic" | "oblique" => Some(FontStyle::Italic),
        _ => None,
    }
}
