use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::context::layout_context::LayoutContext;
use crate::table::table_model::{TableCell, TableModel, TableRow};

const MAX_SPAN: usize = 64;

pub fn collect_table(context: &LayoutContext<'_>, table: NodeId) -> TableModel {
    let document = context.document();
    let styles = context.styles();
    let mut model = TableModel::default();

    let mut row_nodes: Vec<NodeId> = Vec::new();
    for child in document.children(table) {
        match styles.get(child).map(|style| style.display) {
            Some(Display::TableRow) => row_nodes.push(child),
            Some(Display::TableRowGroup) => {
                for row in document.children(child) {
                    if styles.get(row).map(|style| style.display) == Some(Display::TableRow) {
                        row_nodes.push(row);
                    }
                }
            }
            _ => {}
        }
    }

    for row_node in row_nodes {
        let Some(row_style) = styles.get(row_node) else {
            continue;
        };
        let mut cells: Vec<TableCell> = Vec::new();
        let mut column = 0;
        for cell_node in document.children(row_node) {
            let Some(cell_style) = styles.get(cell_node) else {
                continue;
            };
            if cell_style.display != Display::TableCell {
                continue;
            }
            let span = span_of(document, cell_node);
            cells.push(TableCell {
                node: cell_node,
                style: *cell_style,
                column,
                span,
            });
            column += span;
        }
        model.columns = model.columns.max(column);
        model.rows.push(TableRow {
            node: row_node,
            style: *row_style,
            cells,
        });
    }
    model
}

fn span_of(document: &Document, cell: NodeId) -> usize {
    document
        .node(cell)
        .and_then(|node| node.kind().as_element())
        .and_then(|element| element.attribute("colspan"))
        .and_then(|text| text.trim().parse::<usize>().ok())
        .map_or(1, |span| span.clamp(1, MAX_SPAN))
}
