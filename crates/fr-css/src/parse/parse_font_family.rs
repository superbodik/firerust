use cssparser::{Parser, Token};

use crate::value::font_family::FontFamily;

pub fn parse_font_family(parser: &mut Parser<'_>) -> Option<FontFamily> {
    let mut found: Option<FontFamily> = None;
    while !parser.is_exhausted() {
        let token = parser.next().ok()?.clone();
        let name = match token {
            Token::Ident(name) | Token::QuotedString(name) => name.to_ascii_lowercase(),
            _ => continue,
        };
        if found.is_none() {
            found = family_of(&name);
        }
    }
    found
}

fn family_of(name: &str) -> Option<FontFamily> {
    match name {
        "monospace" | "courier" | "courier new" | "consolas" | "menlo" | "monaco"
        | "lucida console" | "ui-monospace" => Some(FontFamily::Monospace),
        "serif" | "times" | "times new roman" | "georgia" | "garamond" | "palatino" => {
            Some(FontFamily::Serif)
        }
        "sans-serif" | "arial" | "helvetica" | "verdana" | "tahoma" | "segoe ui" | "system-ui"
        | "ui-sans-serif" | "roboto" | "inter" => Some(FontFamily::SansSerif),
        _ => None,
    }
}
