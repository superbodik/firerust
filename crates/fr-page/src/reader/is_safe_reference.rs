pub fn is_safe_reference(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lowered = trimmed.to_ascii_lowercase();
    let first_colon = lowered.find(':');
    let first_separator = lowered.find(['/', '?', '#']);
    let has_scheme = match (first_colon, first_separator) {
        (Some(colon), Some(separator)) => colon < separator,
        (Some(_), None) => true,
        _ => false,
    };
    if !has_scheme {
        return true;
    }
    lowered.starts_with("http:") || lowered.starts_with("https:") || lowered.starts_with("mailto:")
}
