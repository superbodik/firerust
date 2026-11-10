use cssparser::{Parser, Token};

use crate::select::selector::Selector;
use crate::select::selector_builder::SelectorBuilder;

pub fn parse_selector_list(prelude: &str) -> Vec<Selector> {
    let mut parser = Parser::new(prelude);
    let mut builder = SelectorBuilder::new();
    let mut selectors: Vec<Selector> = Vec::new();
    while let Ok(token) = parser.next_including_whitespace() {
        let token = token.clone();
        if token == Token::Comma {
            selectors.extend(builder.finish());
        } else {
            builder.accept(&token);
        }
    }
    selectors.extend(builder.finish());
    selectors
}
