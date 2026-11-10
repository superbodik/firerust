use fr_dom::node::node_id::NodeId;

pub fn neighbour_control(
    controls: &[NodeId],
    current: Option<NodeId>,
    forward: bool,
) -> Option<NodeId> {
    if controls.is_empty() {
        return None;
    }
    let position = current.and_then(|id| controls.iter().position(|&control| control == id));
    let index = match (position, forward) {
        (Some(index), true) => (index + 1) % controls.len(),
        (Some(index), false) => (index + controls.len() - 1) % controls.len(),
        (None, true) => 0,
        (None, false) => controls.len() - 1,
    };
    controls.get(index).copied()
}
