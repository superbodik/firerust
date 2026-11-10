use fr_dom::node::node_id::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginFields {
    pub form: Option<NodeId>,
    pub username: Option<NodeId>,
    pub password: NodeId,
}
