use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::text::element_name::element_name;
use crate::text::line_builder::LineBuilder;
use crate::text::tag_class::{has_gap, is_block, is_hidden};

pub fn text_lines(document: &Document) -> Vec<String> {
    let mut builder = LineBuilder::new();
    let mut hidden_root: Option<NodeId> = None;

    for edge in document.walk(document.root()) {
        if let Some(root) = hidden_root {
            if edge == Edge::Leave(root) {
                hidden_root = None;
            }
            continue;
        }
        match edge {
            Edge::Enter(id) => {
                if enter(document, id, &mut builder) {
                    hidden_root = Some(id);
                }
            }
            Edge::Leave(id) => leave(document, id, &mut builder),
        }
    }

    builder.finish()
}

fn enter(document: &Document, id: NodeId, builder: &mut LineBuilder) -> bool {
    let Some(node) = document.node(id) else {
        return false;
    };
    if let Some(text) = node.kind().as_text() {
        builder.push_text(text);
        return false;
    }
    let Some(name) = element_name(document, id) else {
        return false;
    };
    if is_hidden(name) {
        return true;
    }
    if name == "li" {
        builder.start_bullet();
    } else if is_block(name) {
        builder.break_line();
    }
    false
}

fn leave(document: &Document, id: NodeId, builder: &mut LineBuilder) {
    let Some(name) = element_name(document, id) else {
        return;
    };
    if has_gap(name) {
        builder.gap();
    } else if is_block(name) {
        builder.break_line();
    }
}
