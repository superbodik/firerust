use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct TableCell {
    pub node: NodeId,
    pub style: ComputedStyle,
    pub column: usize,
    pub span: usize,
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub node: NodeId,
    pub style: ComputedStyle,
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Default)]
pub struct TableModel {
    pub rows: Vec<TableRow>,
    pub columns: usize,
}
