use url::Url;

use crate::link::classify_link::classify_link;
use crate::link::link_action::LinkAction;

pub fn resolve_link(base: &Url, href: &str) -> Option<Url> {
    match classify_link(base, href, None)? {
        LinkAction::Open { url, .. } => Some(url),
        LinkAction::Scroll { .. } | LinkAction::External { .. } => None,
    }
}
