use cssparser::Parser;

pub fn parse_keyword(parser: &mut Parser<'_>) -> Option<String> {
    Some(parser.expect_ident().ok()?.to_ascii_lowercase())
}
