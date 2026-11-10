use crate::csp::source_expression::SourceExpression;

pub fn parse_source_expression(token: &str) -> Option<SourceExpression> {
    let lowered = token.to_ascii_lowercase();
    match lowered.as_str() {
        "'self'" => return Some(SourceExpression::SelfOrigin),
        "'unsafe-inline'" => return Some(SourceExpression::UnsafeInline),
        "*" => return Some(SourceExpression::Any),
        _ => {}
    }
    if lowered.starts_with('\'') {
        return None;
    }
    if let Some(scheme) = lowered.strip_suffix(':') {
        if !scheme.is_empty() && !scheme.contains('/') {
            return Some(SourceExpression::Scheme(scheme.to_owned()));
        }
        return None;
    }
    let (scheme, rest) = match lowered.split_once("://") {
        Some((scheme, rest)) => (Some(scheme.to_owned()), rest),
        None => (None, lowered.as_str()),
    };
    let rest = rest.split('/').next().unwrap_or("");
    let (host, port) = match rest.rsplit_once(':') {
        Some((host, "*")) => (host, None),
        Some((host, port)) => (host, Some(port.parse::<u16>().ok()?)),
        None => (rest, None),
    };
    let (host, subdomains) = match host.strip_prefix("*.") {
        Some(base) => (base, true),
        None => (host, false),
    };
    if host.is_empty() {
        return None;
    }
    Some(SourceExpression::Host {
        scheme,
        host: host.to_owned(),
        port,
        subdomains,
    })
}
