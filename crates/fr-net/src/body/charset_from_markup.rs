use encoding_rs::{Encoding, UTF_8};

const SNIFF_LENGTH: usize = 2048;

pub fn charset_from_markup(body: &[u8]) -> Option<&'static Encoding> {
    let head = &body[..body.len().min(SNIFF_LENGTH)];
    let text: String = head.iter().map(|&byte| char::from(byte)).collect();
    let lowered = text.to_ascii_lowercase();
    let mut search_from = 0;
    while let Some(found) = lowered[search_from..].find("charset") {
        let start = search_from + found;
        let after = start + "charset".len();
        search_from = after;
        let is_css_rule = start > 0 && lowered.as_bytes()[start - 1] == b'@';
        let trimmed = lowered[after..].trim_start();
        let rest = match trimmed.strip_prefix('=') {
            Some(rest) => rest,
            None if is_css_rule => trimmed,
            None => continue,
        };
        let label: String = rest
            .trim_start()
            .trim_start_matches(['"', '\''])
            .chars()
            .take_while(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | ':' | '.')
            })
            .collect();
        if let Some(encoding) = Encoding::for_label(label.as_bytes()) {
            if encoding == encoding_rs::UTF_16BE || encoding == encoding_rs::UTF_16LE {
                return Some(UTF_8);
            }
            return Some(encoding);
        }
    }
    None
}
