use fr_dom::node::attribute::Attribute;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

pub fn convert_dom(dom: &RcDom) -> Document {
    let mut document = Document::new();
    let mut pending: Vec<(Handle, NodeId)> = vec![(dom.document.clone(), document.root())];

    while let Some((source, parent)) = pending.pop() {
        let children: Vec<Handle> = source.children.borrow().clone();
        for child in children {
            let Some(target) = convert_node(&mut document, &child) else {
                continue;
            };
            if document.append_child(parent, target).is_err() {
                continue;
            }
            pending.push((child, target));
        }
    }

    document
}

fn convert_node(document: &mut Document, source: &Handle) -> Option<NodeId> {
    match &source.data {
        NodeData::Element { name, attrs, .. } => {
            let attributes = attrs
                .borrow()
                .iter()
                .map(|attribute| Attribute::new(&attribute.name.local, &attribute.value))
                .collect();
            document.create_element(&name.local, attributes).ok()
        }
        NodeData::Text { contents } => document.create_text(&contents.borrow()).ok(),
        _ => None,
    }
}
