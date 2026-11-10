use cssparser::Parser;
use fr_geom::color::rgba::Rgba;

use crate::parse::parse_color::parse_color;

pub fn parse_color_text(text: &str) -> Option<Rgba> {
    let trimmed = text.trim();
    let is_bare_hex = matches!(trimmed.len(), 3 | 6)
        && trimmed
            .chars()
            .all(|character| character.is_ascii_hexdigit());
    let candidate = if is_bare_hex {
        format!("#{trimmed}")
    } else {
        trimmed.to_owned()
    };
    let mut parser = Parser::new(&candidate);
    let color = parse_color(&mut parser)?;
    parser.is_exhausted().then_some(color)
}
