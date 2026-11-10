use url::Url;

pub fn resolve_link(base: &Url, href: &str) -> Option<Url> {
    let trimmed = href.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let target = base.join(trimmed).ok()?;
    match target.scheme() {
        "http" | "https" | "file" => Some(target),
        _ => None,
    }
}
