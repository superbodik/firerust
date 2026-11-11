use cssparser::{Parser, Token};

use crate::value::text_decoration::TextDecoration;

pub fn parse_text_decoration(parser: &mut Parser<'_>) -> Option<TextDecoration> {
    let mut decoration = TextDecoration::NONE;
    let mut has_keyword = false;
    while !parser.is_exhausted() {
        let token = parser.next().ok()?.clone();
        if let Token::Ident(name) = token {
            match name.to_ascii_lowercase().as_str() {
                "underline" => {
                    decoration.underline = true;
                    has_keyword = true;
                }
                "line-through" => {
                    decoration.line_through = true;
                    has_keyword = true;
                }
                "none" => has_keyword = true,
                _ => {}
            }
        }
    }
    has_keyword.then_some(decoration)
}
