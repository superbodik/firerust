use crate::error::dom_error::DomError;
use crate::node::attribute::Attribute;
use crate::node::element_data::ElementData;
use crate::node::node_id::NodeId;
use crate::node::node_kind::NodeKind;
use crate::node::tree_node::TreeNode;
use crate::tree::children::Children;
use crate::tree::walk::Walk;

#[derive(Debug, Clone)]
pub struct Document {
    nodes: Vec<TreeNode>,
    revision: u64,
}

impl Document {
    pub fn new() -> Self {
        Self {
            nodes: vec![TreeNode::new(NodeKind::Document)],
            revision: 0,
        }
    }

    pub fn root(&self) -> NodeId {
        NodeId::ROOT
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn id_at(&self, index: usize) -> Option<NodeId> {
        if index < self.nodes.len() {
            NodeId::from_index(index)
        } else {
            None
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node(&self, id: NodeId) -> Option<&TreeNode> {
        self.nodes.get(id.index())
    }

    pub fn children(&self, id: NodeId) -> Children<'_> {
        Children::new(self, self.node(id).and_then(TreeNode::first_child))
    }

    pub fn walk(&self, root: NodeId) -> Walk<'_> {
        Walk::new(self, root)
    }

    pub fn create_element(
        &mut self,
        name: &str,
        attributes: Vec<Attribute>,
    ) -> Result<NodeId, DomError> {
        self.push(NodeKind::Element(ElementData::new(name, attributes)))
    }

    pub fn create_text(&mut self, text: &str) -> Result<NodeId, DomError> {
        self.push(NodeKind::Text(text.into()))
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), DomError> {
        let child_node = self.node(child).ok_or(DomError::UnknownNode)?;
        if child_node.parent.is_some() || child == self.root() {
            return Err(DomError::AlreadyAttached);
        }
        let has_children = child_node.first_child.is_some();
        let previous = self.node(parent).ok_or(DomError::UnknownNode)?.last_child;
        if has_children && self.is_ancestor_or_self(child, parent) {
            return Err(DomError::WouldCreateCycle);
        }
        if parent == child {
            return Err(DomError::WouldCreateCycle);
        }
        self.link(parent, child, previous);
        self.revision += 1;
        Ok(())
    }

    pub fn insert_before(
        &mut self,
        parent: NodeId,
        child: NodeId,
        reference: Option<NodeId>,
    ) -> Result<(), DomError> {
        let Some(reference) = reference else {
            return self.append_child(parent, child);
        };
        let reference_node = self.node(reference).ok_or(DomError::UnknownNode)?;
        if reference_node.parent != Some(parent) {
            return Err(DomError::NotAChild);
        }
        let previous = reference_node.prev_sibling;
        let child_node = self.node(child).ok_or(DomError::UnknownNode)?;
        if child_node.parent.is_some() || child == self.root() {
            return Err(DomError::AlreadyAttached);
        }
        if parent == child || self.is_ancestor_or_self(child, parent) {
            return Err(DomError::WouldCreateCycle);
        }
        if let Some(previous_node) = previous.and_then(|id| self.nodes.get_mut(id.index())) {
            previous_node.next_sibling = Some(child);
        }
        if let Some(child_node) = self.nodes.get_mut(child.index()) {
            child_node.parent = Some(parent);
            child_node.prev_sibling = previous;
            child_node.next_sibling = Some(reference);
        }
        if let Some(reference_node) = self.nodes.get_mut(reference.index()) {
            reference_node.prev_sibling = Some(child);
        }
        if previous.is_none()
            && let Some(parent_node) = self.nodes.get_mut(parent.index())
        {
            parent_node.first_child = Some(child);
        }
        self.revision += 1;
        Ok(())
    }

    pub fn detach(&mut self, child: NodeId) -> Result<(), DomError> {
        let node = self.node(child).ok_or(DomError::UnknownNode)?;
        let Some(parent) = node.parent else {
            return Ok(());
        };
        let (previous, next) = (node.prev_sibling, node.next_sibling);
        if let Some(previous_node) = previous.and_then(|id| self.nodes.get_mut(id.index())) {
            previous_node.next_sibling = next;
        }
        if let Some(next_node) = next.and_then(|id| self.nodes.get_mut(id.index())) {
            next_node.prev_sibling = previous;
        }
        if let Some(parent_node) = self.nodes.get_mut(parent.index()) {
            if previous.is_none() {
                parent_node.first_child = next;
            }
            if next.is_none() {
                parent_node.last_child = previous;
            }
        }
        if let Some(child_node) = self.nodes.get_mut(child.index()) {
            child_node.parent = None;
            child_node.prev_sibling = None;
            child_node.next_sibling = None;
        }
        self.revision += 1;
        Ok(())
    }

    pub fn clear_children(&mut self, id: NodeId) -> Result<(), DomError> {
        let children: Vec<NodeId> = self.children(id).collect();
        for child in children {
            self.detach(child)?;
        }
        Ok(())
    }

    pub fn set_attribute(&mut self, id: NodeId, name: &str, value: &str) -> Result<(), DomError> {
        let node = self
            .nodes
            .get_mut(id.index())
            .ok_or(DomError::UnknownNode)?;
        let element = node.kind.as_element_mut().ok_or(DomError::NotAnElement)?;
        element.set_attribute(name, value);
        self.revision += 1;
        Ok(())
    }

    pub fn remove_attribute(&mut self, id: NodeId, name: &str) -> Result<(), DomError> {
        let node = self
            .nodes
            .get_mut(id.index())
            .ok_or(DomError::UnknownNode)?;
        let element = node.kind.as_element_mut().ok_or(DomError::NotAnElement)?;
        if element.remove_attribute(name) {
            self.revision += 1;
        }
        Ok(())
    }

    pub fn set_text(&mut self, id: NodeId, text: &str) -> Result<(), DomError> {
        let node = self
            .nodes
            .get_mut(id.index())
            .ok_or(DomError::UnknownNode)?;
        if matches!(node.kind, NodeKind::Text(_)) {
            node.kind = NodeKind::Text(text.into());
            self.revision += 1;
            return Ok(());
        }
        self.clear_children(id)?;
        if !text.is_empty() {
            let text_node = self.create_text(text)?;
            self.append_child(id, text_node)?;
        }
        self.revision += 1;
        Ok(())
    }

    fn link(&mut self, parent: NodeId, child: NodeId, previous: Option<NodeId>) {
        if let Some(previous_node) = previous.and_then(|id| self.nodes.get_mut(id.index())) {
            previous_node.next_sibling = Some(child);
        }
        if let Some(child_node) = self.nodes.get_mut(child.index()) {
            child_node.parent = Some(parent);
            child_node.prev_sibling = previous;
        }
        if let Some(parent_node) = self.nodes.get_mut(parent.index()) {
            if parent_node.first_child.is_none() {
                parent_node.first_child = Some(child);
            }
            parent_node.last_child = Some(child);
        }
    }

    fn is_ancestor_or_self(&self, ancestor: NodeId, start: NodeId) -> bool {
        let mut current = Some(start);
        while let Some(id) = current {
            if id == ancestor {
                return true;
            }
            current = self.node(id).and_then(TreeNode::parent);
        }
        false
    }

    fn push(&mut self, kind: NodeKind) -> Result<NodeId, DomError> {
        let id = NodeId::from_index(self.nodes.len()).ok_or(DomError::TooManyNodes)?;
        self.nodes.push(TreeNode::new(kind));
        Ok(id)
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
