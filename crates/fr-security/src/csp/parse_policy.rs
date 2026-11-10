use crate::csp::content_policy::ContentPolicy;
use crate::csp::parse_source_expression::parse_source_expression;
use crate::csp::source_list::SourceList;

pub fn parse_policy(text: &str) -> ContentPolicy {
    let mut policy = ContentPolicy::default();
    for directive in text.split(';') {
        let mut tokens = directive.split_whitespace();
        let Some(name) = tokens.next() else {
            continue;
        };
        let items = tokens
            .filter(|token| !token.eq_ignore_ascii_case("'none'"))
            .filter_map(parse_source_expression)
            .collect();
        let list = SourceList { items };
        let slot = match name.to_ascii_lowercase().as_str() {
            "default-src" => &mut policy.default_src,
            "script-src" => &mut policy.script_src,
            "style-src" => &mut policy.style_src,
            "img-src" => &mut policy.img_src,
            _ => continue,
        };
        if slot.is_none() {
            *slot = Some(list);
        }
    }
    policy
}
