use cssparser::{Parser, Token};

use crate::media::media_matches::media_matches;
use crate::parse::parse_declaration_block::parse_declaration_block;
use crate::parse::rule::Rule;
use crate::parse::stylesheet::Stylesheet;
use crate::select::parse_selector_list::parse_selector_list;

pub fn parse_stylesheet(css: &str) -> Stylesheet {
    let mut parser = Parser::new(css);
    let mut rules: Vec<Rule> = Vec::new();
    parse_rules(&mut parser, &mut rules);
    Stylesheet { rules }
}

fn parse_rules(parser: &mut Parser<'_>, rules: &mut Vec<Rule>) {
    while !parser.is_exhausted() {
        parse_item(parser, rules);
    }
}

fn parse_item(parser: &mut Parser<'_>, rules: &mut Vec<Rule>) {
    parser.skip_whitespace();
    let start = parser.position();
    let mut at_keyword: Option<String> = None;
    loop {
        let Ok(token) = parser.next().cloned() else {
            return;
        };
        match token {
            Token::AtKeyword(name) => at_keyword = Some(name.to_ascii_lowercase()),
            Token::Semicolon => return,
            Token::CurlyBracketBlock => {
                let header = parser
                    .slice_from(start)
                    .trim_end()
                    .trim_end_matches('{')
                    .to_owned();
                match at_keyword {
                    Some(name) => parse_at_rule(&name, &header, parser, rules),
                    None => parse_style_rule(&header, parser, rules),
                }
                return;
            }
            _ => {}
        }
    }
}

fn parse_style_rule(prelude: &str, parser: &mut Parser<'_>, rules: &mut Vec<Rule>) {
    let Ok(declarations) =
        parser.parse_nested_block::<_, _, ()>(|inner| Ok(parse_declaration_block(inner)))
    else {
        return;
    };
    let selectors = parse_selector_list(prelude);
    if selectors.is_empty() {
        return;
    }
    rules.push(Rule {
        selectors,
        declarations,
    });
}

fn parse_at_rule(name: &str, header: &str, parser: &mut Parser<'_>, rules: &mut Vec<Rule>) {
    let condition = header
        .trim_start_matches('@')
        .get(name.len()..)
        .unwrap_or("")
        .trim();
    let is_included = match name {
        "media" => media_matches(condition),
        "supports" => !condition.to_ascii_lowercase().starts_with("not"),
        "layer" => true,
        _ => false,
    };
    if !is_included {
        return;
    }
    let _ = parser.parse_nested_block::<_, _, ()>(|inner| {
        parse_rules(inner, rules);
        Ok(())
    });
}
