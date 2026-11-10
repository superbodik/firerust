use cssparser::{Parser, Token};

use crate::value::length::Length;

pub fn parse_length(parser: &mut Parser<'_>) -> Option<Length> {
    let token = parser.next().ok()?.clone();
    match token {
        Token::Dimension { value, unit, .. } => match unit.to_ascii_lowercase().as_str() {
            "px" => Some(Length::Px(value)),
            "em" => Some(Length::Em(value)),
            "rem" => Some(Length::Rem(value)),
            "pt" => Some(Length::Px(value * 4.0 / 3.0)),
            _ => None,
        },
        Token::Percentage { unit_value, .. } => {
            Some(Length::Percent((unit_value * 10_000.0).round() / 100.0))
        }
        Token::Number { value: 0.0, .. } => Some(Length::Px(0.0)),
        Token::Ident(name) if name.eq_ignore_ascii_case("auto") => Some(Length::Auto),
        _ => None,
    }
}
