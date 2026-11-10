use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::table::collect_table::collect_table;
use crate::table::measure_columns::measure_columns;

pub fn table_widths(context: &mut LayoutContext<'_>, table: NodeId) -> ContentWidths {
    let model = collect_table(context, table);
    let columns = measure_columns(context, &model);
    ContentWidths {
        min: columns.min.iter().sum(),
        max: columns.max.iter().sum(),
    }
}
