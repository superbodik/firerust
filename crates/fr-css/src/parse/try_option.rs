use cssparser::Parser;

pub fn try_option<'i, T>(
    parser: &mut Parser<'i>,
    parse: fn(&mut Parser<'i>) -> Option<T>,
) -> Option<T> {
    parser.try_parse(|inner| parse(inner).ok_or(())).ok()
}
