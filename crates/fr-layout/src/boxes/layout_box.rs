use fr_css::value::edges::Edges;
use fr_dom::node::node_id::NodeId;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;

use crate::boxes::box_kind::BoxKind;

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBox {
    pub node: Option<NodeId>,
    pub kind: BoxKind,
    pub border_box: Rect,
    pub border_width: Edges<f32>,
    pub border_color: Rgba,
    pub background: Rgba,
    pub children: Vec<LayoutBox>,
}

impl Drop for LayoutBox {
    fn drop(&mut self) {
        let mut pending = std::mem::take(&mut self.children);
        while let Some(mut current) = pending.pop() {
            pending.append(&mut current.children);
        }
    }
}

impl LayoutBox {
    pub fn plain(node: Option<NodeId>, kind: BoxKind, border_box: Rect) -> Self {
        Self {
            node,
            kind,
            border_box,
            border_width: Edges::uniform(0.0),
            border_color: Rgba::BLACK,
            background: Rgba::new(0, 0, 0, 0),
            children: Vec::new(),
        }
    }

    pub fn bottom(&self) -> f32 {
        self.border_box.bottom()
    }
}
