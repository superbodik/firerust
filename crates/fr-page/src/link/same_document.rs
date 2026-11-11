use url::{Position, Url};

pub fn is_same_document(first: &Url, second: &Url) -> bool {
    first[..Position::AfterQuery] == second[..Position::AfterQuery]
}
