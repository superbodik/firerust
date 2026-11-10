use crate::csp::content_policy::ContentPolicy;
use crate::fetch::resource_kind::ResourceKind;

pub fn allows_inline(policy: &ContentPolicy, kind: ResourceKind) -> bool {
    let specific = match kind {
        ResourceKind::Script => &policy.script_src,
        ResourceKind::Style => &policy.style_src,
        ResourceKind::Image => return true,
    };
    match specific.as_ref().or(policy.default_src.as_ref()) {
        Some(list) => list.allows_inline(),
        None => true,
    }
}
