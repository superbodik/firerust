use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use url::Url;

pub fn linked_sheet_url(document: &Document, id: NodeId, base: &Url) -> Option<Url> {
    let data = document.node(id)?.kind().as_element()?;
    let is_stylesheet = data
        .attribute("rel")?
        .split_whitespace()
        .any(|part| part.eq_ignore_ascii_case("stylesheet"));
    let is_print_only = data
        .attribute("media")
        .is_some_and(|media| media.to_ascii_lowercase().contains("print"));
    if !is_stylesheet || is_print_only {
        return None;
    }
    base.join(data.attribute("href")?).ok()
}
