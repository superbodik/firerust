use url::Url;

use crate::csp::content_policy::ContentPolicy;
use crate::csp::source_matches::source_matches;
use crate::fetch::resource_kind::ResourceKind;

pub fn allows_resource(
    policy: &ContentPolicy,
    kind: ResourceKind,
    page: &Url,
    target: &Url,
) -> bool {
    let specific = match kind {
        ResourceKind::Script => &policy.script_src,
        ResourceKind::Style => &policy.style_src,
        ResourceKind::Image => &policy.img_src,
    };
    let Some(list) = specific.as_ref().or(policy.default_src.as_ref()) else {
        return true;
    };
    list.items
        .iter()
        .any(|source| source_matches(source, page, target))
}
