use cssparser::Parser;

use crate::parse::declaration::Declaration;
use crate::parse::parse_property::parse_property;

pub fn parse_property_text(name: &str, text: &str) -> Option<Vec<Declaration>> {
    let mut parser = Parser::new(text);
    let parsed = parse_property(name, &mut parser)?;
    parser.is_exhausted().then_some(parsed)
}
