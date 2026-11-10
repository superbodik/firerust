use cssparser::{ParseError, Parser, Token};

use crate::select::attribute_condition::AttributeCondition;
use crate::select::attribute_operator::AttributeOperator;

pub fn parse_attribute_condition<'i>(
    parser: &mut Parser<'i>,
) -> Result<AttributeCondition, ParseError<()>> {
    parser.skip_whitespace();
    let name = match parser.next()?.clone() {
        Token::Ident(name) => name.to_ascii_lowercase().into_boxed_str(),
        _ => return Err(ParseError::custom(())),
    };
    parser.skip_whitespace();
    let operator = match parser.next() {
        Err(_) => {
            return Ok(AttributeCondition {
                name,
                operator: AttributeOperator::Exists,
                value: "".into(),
                ignores_case: false,
            });
        }
        Ok(Token::Delim('=')) => AttributeOperator::Equals,
        Ok(Token::IncludeMatch) => AttributeOperator::Includes,
        Ok(Token::DashMatch) => AttributeOperator::DashMatch,
        Ok(Token::PrefixMatch) => AttributeOperator::Prefix,
        Ok(Token::SuffixMatch) => AttributeOperator::Suffix,
        Ok(Token::SubstringMatch) => AttributeOperator::Substring,
        Ok(_) => return Err(ParseError::custom(())),
    };
    parser.skip_whitespace();
    let value = match parser.next()?.clone() {
        Token::Ident(text) | Token::QuotedString(text) => text.to_string().into_boxed_str(),
        Token::Number {
            int_value: Some(number),
            ..
        } => number.to_string().into_boxed_str(),
        _ => return Err(ParseError::custom(())),
    };
    parser.skip_whitespace();
    let ignores_case = match parser.next() {
        Ok(Token::Ident(flag)) => flag.eq_ignore_ascii_case("i"),
        _ => false,
    };
    Ok(AttributeCondition {
        name,
        operator,
        value,
        ignores_case,
    })
}
