use cssparser::{Parser, Token};

use crate::select::parse_attribute_condition::parse_attribute_condition;
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
            other => builder.accept(&other),
        }
    }
    selectors.extend(builder.finish());
    selectors
}
