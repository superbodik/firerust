use fr_dom::node::node_id::NodeId;

use crate::cascade::computed_style::ComputedStyle;

#[derive(Debug, Clone)]
pub struct StyleMap {
    styles: Vec<Option<ComputedStyle>>,
}

impl StyleMap {
    pub fn new(node_count: usize) -> Self {
        Self {
            styles: vec![None; node_count],
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&ComputedStyle> {
        self.styles.get(id.index())?.as_ref()
    }

    pub fn set(&mut self, id: NodeId, style: ComputedStyle) {
        if let Some(slot) = self.styles.get_mut(id.index()) {
            *slot = Some(style);
        }
    }
}
