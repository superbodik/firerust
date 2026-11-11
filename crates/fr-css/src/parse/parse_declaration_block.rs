use cssparser::{Delimiter, ParseError, Parser};

use crate::parse::declaration::Declaration;
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
    let declarations = parser.parse_until_before(Delimiter::Bang, |value| {
        if name.starts_with("--") {
            let text = raw_value(value);
            return Ok(vec![Declaration::Custom(name.as_str().into(), text.into())]);
        }
        let start = value.position();
        match parse_property(&name, value) {
            Some(parsed) if value.is_exhausted() => Ok(parsed),
            _ => {
                while value.next_including_whitespace().is_ok() {}
                let text = value.slice_from(start).trim();
                if text.to_ascii_lowercase().contains("var(") {
                    Ok(vec![Declaration::Deferred(
                        name.as_str().into(),
                        text.into(),
                    )])
                } else {
                    Err(ParseError::custom(()))
                }
            }
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

fn raw_value(parser: &mut Parser<'_>) -> String {
    let start = parser.position();
    while parser.next_including_whitespace().is_ok() {}
    parser.slice_from(start).trim().to_owned()
}
