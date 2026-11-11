use std::path::{Path, absolute};

use crate::address::input_kind::InputKind;

const ADDRESS_SCHEMES: [&str; 5] = ["http", "https", "file", "about", "ftp"];

pub fn classify_input(input: &str, base_dir: &Path) -> InputKind {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return InputKind::Address;
    }
    if has_known_scheme(trimmed) || is_existing_file(trimmed, base_dir) || is_drive_path(trimmed) {
        return InputKind::Address;
    }
    if trimmed.chars().any(char::is_whitespace) {
        return InputKind::Search;
    }
    if looks_like_host(host_part(trimmed)) {
        InputKind::Address
    } else {
        InputKind::Search
    }
}

fn has_known_scheme(text: &str) -> bool {
    let Some((scheme, rest)) = text.split_once(':') else {
        return false;
    };
    let scheme = scheme.to_ascii_lowercase();
    if !ADDRESS_SCHEMES.contains(&scheme.as_str()) {
        return false;
    }
    scheme == "about" || rest.starts_with("//")
}

fn is_existing_file(text: &str, base_dir: &Path) -> bool {
    absolute(base_dir.join(text)).is_ok_and(|path| path.is_file())
}

fn is_drive_path(text: &str) -> bool {
    let mut characters = text.chars();
    let (Some(letter), Some(colon), Some(slash)) =
        (characters.next(), characters.next(), characters.next())
    else {
        return false;
    };
    letter.is_ascii_alphabetic() && colon == ':' && matches!(slash, '\\' | '/')
}

fn host_part(text: &str) -> &str {
    let end = text.find(['/', '?', '#']).unwrap_or(text.len());
    let authority = &text[..end];
    let without_user = authority.rsplit('@').next().unwrap_or(authority);
    if without_user.starts_with('[') {
        return without_user;
    }
    without_user.split(':').next().unwrap_or(without_user)
}

fn looks_like_host(host: &str) -> bool {
    if host.starts_with('[') {
        return true;
    }
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }
    if !host.contains('.') {
        return false;
    }
    let labels: Vec<&str> = host.split('.').collect();
    if labels.iter().any(|label| label.is_empty()) {
        return false;
    }
    let is_ip = labels.iter().all(|label| {
        !label.is_empty() && label.chars().all(|character| character.is_ascii_digit())
    });
    if is_ip {
        return true;
    }
    labels
        .last()
        .is_some_and(|top| top.chars().count() >= 2 && top.chars().all(char::is_alphanumeric))
}
