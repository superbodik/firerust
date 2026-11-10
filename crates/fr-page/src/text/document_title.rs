use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

pub fn document_title(document: &Document) -> Option<String> {
    let title = document.walk(document.root()).find_map(|edge| match edge {
        Edge::Enter(id) if element_name(document, id) == Some("title") => Some(id),
        _ => None,
    })?;
    let raw: String = document
        .children(title)
        .filter_map(|child| document.node(child)?.kind().as_text())
        .collect();
    let collapsed = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        None
    } else {
        Some(collapsed)
    }
}
