use cssparser::{Parser, Token};

use crate::parse::parse_declaration_block::parse_declaration_block;
use crate::parse::rule::Rule;
use crate::parse::stylesheet::Stylesheet;
use crate::select::parse_selector_list::parse_selector_list;

pub fn parse_stylesheet(css: &str) -> Stylesheet {
    let mut parser = Parser::new(css);
    let mut rules: Vec<Rule> = Vec::new();
    while !parser.is_exhausted() {
        if let Some(rule) = parse_rule(&mut parser) {
            rules.push(rule);
        }
    }
    Stylesheet { rules }
}

fn parse_rule(parser: &mut Parser<'_>) -> Option<Rule> {
    parser.skip_whitespace();
    let start = parser.position();
    let mut is_at_rule = false;
    loop {
        let token = parser.next().ok()?.clone();
        match token {
            Token::AtKeyword(_) => is_at_rule = true,
            Token::Semicolon => return None,
            Token::CurlyBracketBlock => {
                if is_at_rule {
                    return None;
                }
                let prelude = parser
                    .slice_from(start)
                    .trim_end()
                    .trim_end_matches('{')
                    .to_owned();
                let declarations = parser
                    .parse_nested_block::<_, _, ()>(|inner| Ok(parse_declaration_block(inner)))
                    .ok()?;
                let selectors = parse_selector_list(&prelude);
                if selectors.is_empty() {
                    return None;
                }
                return Some(Rule {
                    selectors,
                    declarations,
                });
            }
            _ => {}
        }
    }
}
