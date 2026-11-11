use crate::media::query_matches::query_matches;
use crate::media::split_top_level::split_top_level;

pub fn media_matches(prelude: &str) -> bool {
    if prelude.trim().is_empty() {
        return true;
    }
    split_top_level(prelude, ",").into_iter().any(query_matches)
}
