use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

pub fn needs_scripting(document: &Document) -> bool {
    document.walk(document.root()).any(|edge| {
        let Edge::Enter(id) = edge else {
            return false;
        };
        let Some(element) = document.node(id).and_then(|node| node.kind().as_element()) else {
            return false;
        };
        element.name() == "script"
            || element
                .attributes()
                .iter()
                .any(|attribute| attribute.name().len() > 2 && attribute.name().starts_with("on"))
    })
}
