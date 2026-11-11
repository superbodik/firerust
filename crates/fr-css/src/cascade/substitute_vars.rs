use crate::cascade::custom_properties::CustomProperties;

const MAX_DEPTH: usize = 12;
const MAX_LENGTH: usize = 8_192;

pub fn substitute_vars(text: &str, vars: &CustomProperties) -> Option<String> {
    substitute(text, vars, 0)
}

fn substitute(text: &str, vars: &CustomProperties, depth: usize) -> Option<String> {
    if depth > MAX_DEPTH {
        return None;
    }
    let mut result = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = find_var(rest) {
        result.push_str(&rest[..start]);
        let after = &rest[start + 4..];
        let end = matching_paren(after)?;
        let inner = &after[..end];
        let (name, fallback) = split_arguments(inner);
        let value = match vars.get(name.trim()) {
            Some(found) => substitute(found, vars, depth + 1)?,
            None => substitute(fallback?, vars, depth + 1)?,
        };
        result.push_str(value.trim());
        rest = &after[end + 1..];
        if result.len() > MAX_LENGTH {
            return None;
        }
    }
    result.push_str(rest);
    Some(result)
}

fn find_var(text: &str) -> Option<usize> {
    let lower = text.to_ascii_lowercase();
    lower.find("var(")
}

fn matching_paren(text: &str) -> Option<usize> {
    let mut depth = 1usize;
    for (index, character) in text.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn split_arguments(inner: &str) -> (&str, Option<&str>) {
    let mut depth = 0usize;
    for (index, character) in inner.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => return (&inner[..index], Some(&inner[index + 1..])),
            _ => {}
        }
    }
    (inner, None)
}
