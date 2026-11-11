use cssparser::{ParseError, Parser, parse_nth};

use crate::select::parse_selector_list::parse_selector_list;
use crate::select::pseudo_class::PseudoClass;

pub fn parse_functional_pseudo(
    name: &str,
    parser: &mut Parser<'_>,
) -> Result<PseudoClass, ParseError<()>> {
    match name {
        "nth-child" => {
            let (step, offset) = parse_nth(parser).map_err(|_| ParseError::custom(()))?;
            Ok(PseudoClass::NthChild(step, offset))
        }
        "nth-last-child" => {
            let (step, offset) = parse_nth(parser).map_err(|_| ParseError::custom(()))?;
            Ok(PseudoClass::NthLastChild(step, offset))
        }
        "not" => {
            let start = parser.position();
            while parser.next_including_whitespace().is_ok() {}
            let text = parser.slice_from(start).to_owned();
            let mut compounds = Vec::new();
            for selector in parse_selector_list(&text) {
                let [part] = selector.parts.as_slice() else {
                    return Err(ParseError::custom(()));
                };
                compounds.push(part.compound.clone());
            }
            if compounds.is_empty() {
                return Err(ParseError::custom(()));
            }
            Ok(PseudoClass::Not(compounds))
        }
        _ => Err(ParseError::custom(())),
    }
}
