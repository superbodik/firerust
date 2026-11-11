pub fn split_top_level<'a>(text: &'a str, separator: &str) -> Vec<&'a str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    let mut index = 0;
    while index < text.len() {
        let rest = &text[index..];
        if depth == 0 && rest.starts_with(separator) {
            parts.push(&text[start..index]);
            index += separator.len();
            start = index;
            continue;
        }
        let Some(character) = rest.chars().next() else {
            break;
        };
        match character {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ => {}
        }
        index += character.len_utf8();
    }
    parts.push(&text[start..]);
    parts
}
