use url::Url;

use crate::csp::source_expression::SourceExpression;
use crate::origin::same_origin::same_origin;

pub fn source_matches(source: &SourceExpression, page: &Url, target: &Url) -> bool {
    match source {
        SourceExpression::SelfOrigin => same_origin(page, target) || both_files(page, target),
        SourceExpression::UnsafeInline => false,
        SourceExpression::Any => matches!(target.scheme(), "http" | "https"),
        SourceExpression::Scheme(scheme) => target.scheme() == scheme,
        SourceExpression::Host {
            scheme,
            host,
            port,
            subdomains,
        } => {
            let Some(target_host) = target.host_str() else {
                return false;
            };
            let target_host = target_host.to_ascii_lowercase();
            let host_ok = if *subdomains {
                target_host.ends_with(&format!(".{host}"))
            } else {
                target_host == *host
            };
            let scheme_ok = match scheme {
                Some(scheme) => target.scheme() == scheme,
                None => matches!(target.scheme(), "http" | "https"),
            };
            let port_ok = port.is_none_or(|wanted| target.port_or_known_default() == Some(wanted));
            host_ok && scheme_ok && port_ok
        }
    }
}

fn both_files(page: &Url, target: &Url) -> bool {
    page.scheme() == "file" && target.scheme() == "file"
}
