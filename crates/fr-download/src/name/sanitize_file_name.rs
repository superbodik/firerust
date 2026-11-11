const FALLBACK: &str = "download";
const MAX_LENGTH: usize = 120;
const FORBIDDEN: [char; 9] = ['<', '>', ':', '"', '|', '?', '*', '/', '\\'];
const RESERVED: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

pub fn sanitize_file_name(raw: &str) -> String {
    let last_segment = raw.rsplit(['/', '\\']).next().unwrap_or("");
    let cleaned: String = last_segment
        .chars()
        .filter(|character| !character.is_control() && !FORBIDDEN.contains(character))
        .collect();
    let trimmed = cleaned.trim().trim_end_matches(['.', ' ']);
    if trimmed.is_empty() || trimmed.chars().all(|character| character == '.') {
        return FALLBACK.to_owned();
    }
    let shortened = shorten(trimmed);
    if is_reserved(&shortened) {
        return format!("_{shortened}");
    }
    shortened
}

fn shorten(name: &str) -> String {
    if name.chars().count() <= MAX_LENGTH {
        return name.to_owned();
    }
    match name.rfind('.') {
        Some(dot) if name.len() - dot <= 12 => {
            let extension = &name[dot..];
            let keep = MAX_LENGTH.saturating_sub(extension.chars().count());
            let stem: String = name[..dot].chars().take(keep).collect();
            format!("{stem}{extension}")
        }
        _ => name.chars().take(MAX_LENGTH).collect(),
    }
}

fn is_reserved(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or("").to_ascii_uppercase();
    RESERVED.contains(&stem.as_str())
}
