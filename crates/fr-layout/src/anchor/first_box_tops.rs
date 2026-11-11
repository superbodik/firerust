use std::collections::HashMap;

use fr_dom::node::node_id::NodeId;

use crate::boxes::layout_box::LayoutBox;

pub fn first_box_tops(root: &LayoutBox) -> HashMap<NodeId, f32> {
    let mut tops: HashMap<NodeId, f32> = HashMap::new();
    let mut pending: Vec<&LayoutBox> = vec![root];
    while let Some(current) = pending.pop() {
        if let Some(node) = current.node {
            tops.entry(node).or_insert(current.border_box.origin.y);
        }
        pending.extend(current.children.iter().rev());
    }
    tops
}
