use crate::node::node_id::NodeId;
use crate::tree::document::Document;
use crate::tree::edge::Edge;

pub struct Walk<'a> {
    document: &'a Document,
    root: NodeId,
    next: Option<Edge>,
}

impl<'a> Walk<'a> {
    pub(crate) fn new(document: &'a Document, root: NodeId) -> Self {
        let first = document.node(root).map(|_| Edge::Enter(root));
        Self {
            document,
            root,
            next: first,
        }
    }

    fn following(&self, edge: Edge) -> Option<Edge> {
        match edge {
            Edge::Enter(id) => {
                let node = self.document.node(id)?;
                Some(match node.first_child() {
                    Some(child) => Edge::Enter(child),
                    None => Edge::Leave(id),
                })
            }
            Edge::Leave(id) => {
                if id == self.root {
                    return None;
                }
                let node = self.document.node(id)?;
                Some(match node.next_sibling() {
                    Some(sibling) => Edge::Enter(sibling),
                    None => Edge::Leave(node.parent()?),
                })
            }
        }
    }
}

impl Iterator for Walk<'_> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        let current = self.next?;
        self.next = self.following(current);
        Some(current)
    }
}
