use cssparser::{Delimiter, ParseError, Parser};

use crate::parse::parse_property::parse_property;
use crate::parse::style_declaration::StyleDeclaration;

pub fn parse_declaration_block(parser: &mut Parser<'_>) -> Vec<StyleDeclaration> {
    let mut result: Vec<StyleDeclaration> = Vec::new();
    while !parser.is_exhausted() {
        if let Ok(mut parsed) =
            parser.parse_until_after(Delimiter::Semicolon, |inner| parse_declaration(inner))
        {
            result.append(&mut parsed);
        }
    }
    result
}

fn parse_declaration(parser: &mut Parser<'_>) -> Result<Vec<StyleDeclaration>, ParseError<()>> {
    let name = parser.expect_ident()?.to_ascii_lowercase();
    parser.expect_colon()?;
    let declarations =
        parser.parse_until_before(Delimiter::Bang, |value| {
            match parse_property(&name, value) {
                Some(parsed) if value.is_exhausted() => Ok(parsed),
                _ => Err(ParseError::custom(())),
            }
        })?;
    let is_important = parser
        .try_parse(|inner| {
            inner.expect_delim('!')?;
            inner.expect_ident_matching("important")
        })
        .is_ok();
    parser.expect_exhausted()?;
    Ok(declarations
        .into_iter()
        .map(|declaration| StyleDeclaration {
            declaration,
            is_important,
        })
        .collect())
}
