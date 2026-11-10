use cssparser::{ParseError, Parser, Token};
use fr_geom::color::rgba::Rgba;

use crate::value::named_color::named_color;

pub fn parse_color(parser: &mut Parser<'_>) -> Option<Rgba> {
    let token = parser.next().ok()?.clone();
    match token {
        Token::Hash(hex) | Token::IDHash(hex) => parse_hex(&hex),
        Token::Ident(name) => named_color(&name),
        Token::Function(name)
            if name.eq_ignore_ascii_case("rgb") || name.eq_ignore_ascii_case("rgba") =>
        {
            parser
                .parse_nested_block(|inner| {
                    parse_rgb_arguments(inner).ok_or(ParseError::<()>::custom(()))
                })
                .ok()
        }
        _ => None,
    }
}

fn parse_hex(hex: &str) -> Option<Rgba> {
    let digits: Vec<u8> = hex
        .chars()
        .map(|character| character.to_digit(16).map(|digit| digit as u8))
        .collect::<Option<_>>()?;
    match digits.as_slice() {
        [r, g, b] => Some(Rgba::opaque(r * 17, g * 17, b * 17)),
        [r, g, b, a] => Some(Rgba::new(r * 17, g * 17, b * 17, a * 17)),
        [r1, r2, g1, g2, b1, b2] => Some(Rgba::opaque(r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2)),
        [r1, r2, g1, g2, b1, b2, a1, a2] => Some(Rgba::new(
            r1 * 16 + r2,
            g1 * 16 + g2,
            b1 * 16 + b2,
            a1 * 16 + a2,
        )),
        _ => None,
    }
}

fn parse_rgb_arguments(parser: &mut Parser<'_>) -> Option<Rgba> {
    let mut components: Vec<(f32, bool)> = Vec::new();
    while !parser.is_exhausted() {
        let token = parser.next().ok()?.clone();
        match token {
            Token::Number { value, .. } => components.push((value, false)),
            Token::Percentage { unit_value, .. } => components.push((unit_value, true)),
            Token::Comma | Token::Delim('/') => {}
            _ => return None,
        }
    }
    let channel = |index: usize| -> Option<u8> {
        let (value, is_percent) = *components.get(index)?;
        let scaled = if is_percent { value * 255.0 } else { value };
        Some(scaled.round().clamp(0.0, 255.0) as u8)
    };
    let alpha = match components.get(3) {
        Some(&(value, true)) => (value * 255.0).round().clamp(0.0, 255.0) as u8,
        Some(&(value, false)) => (value * 255.0).round().clamp(0.0, 255.0) as u8,
        None => 255,
    };
    if components.len() != 3 && components.len() != 4 {
        return None;
    }
    Some(Rgba::new(channel(0)?, channel(1)?, channel(2)?, alpha))
}
