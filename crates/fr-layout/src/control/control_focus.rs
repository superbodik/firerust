use fr_dom::node::node_id::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlFocus {
    pub node: NodeId,
    pub caret: usize,
}
