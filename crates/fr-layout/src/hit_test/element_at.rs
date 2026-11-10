use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::point::Point;

use crate::boxes::layout_box::LayoutBox;

pub fn element_at(root: &LayoutBox, point: Point) -> Option<NodeId> {
    let mut current = root;
    let mut found: Option<NodeId> = None;
    loop {
        if !current.border_box.contains(point) && !std::ptr::eq(current, root) {
            break;
        }
        if current.node.is_some() {
            found = current.node;
        }
        match current
            .children
            .iter()
            .rev()
            .find(|child| child.border_box.contains(point))
        {
            Some(child) => current = child,
            None => break,
        }
    }
    found
}
