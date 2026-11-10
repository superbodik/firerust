use crate::node::node_id::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Enter(NodeId),
    Leave(NodeId),
}
