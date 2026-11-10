use fr_dom::node::node_id::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlAction {
    Focus(NodeId),
    Changed(NodeId),
    Submit { form: NodeId, submitter: NodeId },
    Nothing,
}
