use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;

#[derive(Debug, Clone)]
pub enum FlexItem {
    Element {
        node: NodeId,
        style: ComputedStyle,
    },
    Anonymous {
        nodes: Vec<NodeId>,
        style: ComputedStyle,
    },
}

impl FlexItem {
    pub fn style(&self) -> &ComputedStyle {
        match self {
            Self::Element { style, .. } | Self::Anonymous { style, .. } => style,
        }
    }
}
