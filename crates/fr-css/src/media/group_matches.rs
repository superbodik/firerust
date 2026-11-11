use crate::media::feature_matches::feature_matches;
use crate::media::range_matches::range_matches;

pub fn group_matches(group: &str) -> bool {
    let trimmed = group.trim();
    let inner = trimmed
        .strip_prefix('(')
        .and_then(|text| text.strip_suffix(')'))
        .unwrap_or(trimmed)
        .trim();
    if let Some((name, value)) = inner.split_once(':') {
        return feature_matches(name.trim(), value);
    }
    if inner.contains(['<', '>', '=']) {
        return range_matches(inner);
    }
    matches!(
        inner,
        "color" | "hover" | "pointer" | "any-hover" | "any-pointer"
    )
}
