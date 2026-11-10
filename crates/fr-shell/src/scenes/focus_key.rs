use fr_dom::node::node_id::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusKey {
    Edited,
    SubmitFrom(NodeId),
    Activate(NodeId),
    Blurred,
    Ignored,
}
