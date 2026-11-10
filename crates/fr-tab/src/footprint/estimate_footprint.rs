use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::tree_node::TreeNode;

use crate::footprint::layout_footprint::layout_footprint;
use crate::footprint::text_footprint::text_footprint;
use crate::model::tab_content::TabContent;

pub fn estimate_footprint(content: &TabContent) -> usize {
    let nodes = content.document.node_count();
    let tree = nodes * std::mem::size_of::<TreeNode>() + text_footprint(&content.document);
    let styles = nodes * std::mem::size_of::<Option<ComputedStyle>>();
    let layout = content.layout.as_ref().map_or(0, layout_footprint);
    tree + styles + layout + content.images.byte_len()
}
