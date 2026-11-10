use cssparser::{Parser, Token};

use crate::value::line_height::LineHeight;

pub fn parse_line_height(parser: &mut Parser<'_>) -> Option<LineHeight> {
    let token = parser.next().ok()?.clone();
    match token {
        Token::Ident(name) if name.eq_ignore_ascii_case("normal") => Some(LineHeight::Normal),
        Token::Number { value, .. } if value >= 0.0 => Some(LineHeight::Number(value)),
        Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
            Some(LineHeight::Number(unit_value))
        }
        Token::Dimension { value, unit, .. } if value >= 0.0 => {
            match unit.to_ascii_lowercase().as_str() {
                "px" => Some(LineHeight::Px(value)),
                "pt" => Some(LineHeight::Px(value * 4.0 / 3.0)),
                "em" | "rem" => Some(LineHeight::Number(value)),
                _ => None,
            }
        }
        _ => None,
    }
}
