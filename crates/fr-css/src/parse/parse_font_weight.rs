use cssparser::{Parser, Token};

pub fn parse_font_weight(parser: &mut Parser<'_>) -> Option<u16> {
    let token = parser.next().ok()?.clone();
    match token {
        Token::Ident(name) if name.eq_ignore_ascii_case("normal") => Some(400),
        Token::Ident(name) if name.eq_ignore_ascii_case("bold") => Some(700),
        Token::Number { value, .. } if (1.0..=1000.0).contains(&value) => {
            Some(value.round() as u16)
        }
        _ => None,
    }
}
