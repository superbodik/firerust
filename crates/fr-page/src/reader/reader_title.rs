use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::text::document_title::document_title;

pub fn reader_title(document: &Document) -> String {
    if let Some(title) = document_title(document) {
        return title;
    }
    let heading = document.walk(document.root()).find_map(|edge| match edge {
        Edge::Enter(id) if element_name(document, id) == Some("h1") => Some(id),
        _ => None,
    });
    let from_heading = heading.map(|id| {
        let mut text = String::new();
        for edge in document.walk(id) {
            if let Edge::Enter(inner) = edge
                && let Some(piece) = document.node(inner).and_then(|node| node.kind().as_text())
            {
                text.push_str(piece);
            }
        }
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    });
    from_heading
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "Reader view".to_owned())
}
