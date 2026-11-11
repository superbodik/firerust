use fr_net::address::percent_decode::percent_decode;
use fr_security::navigation::check_navigation::check_navigation;
use url::Url;

use crate::link::external_url::external_url;
use crate::link::link_action::LinkAction;
use crate::link::same_document::is_same_document;

pub fn classify_link(base: &Url, href: &str, target: Option<&str>) -> Option<LinkAction> {
    let trimmed = href.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(fragment) = trimmed.strip_prefix('#') {
        return Some(LinkAction::Scroll {
            fragment: percent_decode(fragment),
        });
    }
    let joined = base.join(trimmed).ok()?;
    if is_same_document(base, &joined)
        && let Some(fragment) = joined.fragment()
    {
        return Some(LinkAction::Scroll {
            fragment: percent_decode(fragment),
        });
    }
    if matches!(joined.scheme(), "mailto" | "tel") {
        return external_url(&joined).map(|url| LinkAction::External { url });
    }
    match check_navigation(base, &joined) {
        Ok(()) => Some(LinkAction::Open {
            url: joined,
            in_new_tab: opens_in_new_tab(target),
        }),
        Err(reason) => {
            eprintln!("blocked link {joined}: {reason}");
            None
        }
    }
}

fn opens_in_new_tab(target: Option<&str>) -> bool {
    target.is_some_and(|name| name.trim().eq_ignore_ascii_case("_blank"))
}
