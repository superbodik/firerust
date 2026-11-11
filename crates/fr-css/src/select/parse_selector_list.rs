use cssparser::{Parser, Token};

use crate::select::parse_attribute_condition::parse_attribute_condition;
use crate::select::parse_functional_pseudo::parse_functional_pseudo;
use crate::select::pseudo_from_name::pseudo_from_name;
use crate::select::selector::Selector;
use crate::select::selector_builder::SelectorBuilder;

pub fn parse_selector_list(prelude: &str) -> Vec<Selector> {
    let mut parser = Parser::new(prelude);
    let mut builder = SelectorBuilder::new();
    let mut selectors: Vec<Selector> = Vec::new();
    while let Ok(token) = parser.next_including_whitespace() {
        let token = token.clone();
        match token {
            Token::Comma => selectors.extend(builder.finish()),
            Token::SquareBracketBlock => {
                match parser.parse_nested_block(|inner| parse_attribute_condition(inner)) {
                    Ok(condition) => builder.accept_attribute(condition),
                    Err(_) => builder.invalidate(),
                }
            }
            Token::Colon => match parser.next_including_whitespace().cloned() {
                Ok(Token::Ident(name)) => match pseudo_from_name(&name) {
                    Some(pseudo) => builder.accept_pseudo(pseudo),
                    None => builder.invalidate(),
                },
                Ok(Token::Function(name)) => {
                    let name = name.to_ascii_lowercase();
                    match parser.parse_nested_block(|inner| parse_functional_pseudo(&name, inner)) {
                        Ok(pseudo) => builder.accept_pseudo(pseudo),
                        Err(_) => builder.invalidate(),
                    }
                }
                _ => builder.invalidate(),
            },
            other => builder.accept(&other),
        }
    }
    selectors.extend(builder.finish());
    selectors
}
