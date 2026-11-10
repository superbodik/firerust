use fr_security::navigation::check_navigation::check_navigation;
use url::Url;

pub fn resolve_link(base: &Url, href: &str) -> Option<Url> {
    let trimmed = href.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let target = base.join(trimmed).ok()?;
    match check_navigation(base, &target) {
        Ok(()) => Some(target),
        Err(reason) => {
            eprintln!("blocked link {target}: {reason}");
            None
        }
    }
}
