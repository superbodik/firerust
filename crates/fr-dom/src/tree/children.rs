use crate::node::node_id::NodeId;
use crate::node::tree_node::TreeNode;
use crate::tree::document::Document;

pub struct Children<'a> {
    document: &'a Document,
    next: Option<NodeId>,
}

impl<'a> Children<'a> {
    pub(crate) fn new(document: &'a Document, first: Option<NodeId>) -> Self {
        Self {
            document,
            next: first,
        }
    }
}

impl Iterator for Children<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        let current = self.next?;
        self.next = self.document.node(current).and_then(TreeNode::next_sibling);
        Some(current)
    }
}
