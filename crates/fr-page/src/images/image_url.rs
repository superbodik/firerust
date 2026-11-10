use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use url::Url;

pub fn image_url(document: &Document, id: NodeId, base: &Url) -> Option<Url> {
    let source = document
        .node(id)?
        .kind()
        .as_element()?
        .attribute("src")?
        .trim();
    if source.is_empty() {
        return None;
    }
    base.join(source).ok()
}
