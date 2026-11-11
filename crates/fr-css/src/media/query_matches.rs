use crate::media::group_matches::group_matches;
use crate::media::split_top_level::split_top_level;

pub fn query_matches(query: &str) -> bool {
    let lowered = query.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return true;
    }
    let mut is_negated = false;
    let mut is_satisfied = true;
    for part in split_top_level(&lowered, " and ") {
        let part = part.trim();
        if part.starts_with('(') {
            is_satisfied &= group_matches(part);
            continue;
        }
        for word in part.split_whitespace() {
            match word {
                "not" => is_negated = true,
                "only" | "screen" | "all" => {}
                _ => is_satisfied = false,
            }
        }
    }
    is_satisfied != is_negated
}
