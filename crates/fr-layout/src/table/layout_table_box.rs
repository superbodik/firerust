use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::box_sizing::BoxSizing;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::rect::Rect;

use crate::block::block_result::BlockResult;
use crate::block::block_spec::BlockSpec;
use crate::block::layout_block_box::layout_block_box;
use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::image::attr_length::attr_length;
use crate::metrics::resolve_length::resolve_length;
use crate::metrics::resolve_metrics::resolve_metrics;
use crate::metrics::resolve_radii::resolve_radii;
use crate::table::collect_table::collect_table;
use crate::table::distribute_columns::distribute_columns;
use crate::table::measure_columns::measure_columns;
use crate::table::table_model::TableRow;

pub fn layout_table_box(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    style: &ComputedStyle,
    spec: BlockSpec,
) -> BlockResult {
    let metrics = resolve_metrics(style, spec.available_width);
    let extras = metrics.horizontal_extras();
    let outer_available = spec.available_width - metrics.margin.left - metrics.margin.right;
    let content_available = (outer_available - extras).max(0.0);

    let model = collect_table(context, node);
    let columns = measure_columns(context, &model);
    let total_max: f32 = columns.max.iter().sum();

    let specified = resolve_length(style.size.width, spec.available_width).or_else(|| {
        attr_length(context.document(), node, "width")
            .and_then(|length| resolve_length(length, spec.available_width))
    });
    let (target, grows) = match specified {
        Some(width) => {
            let inner = match style.box_sizing {
                BoxSizing::BorderBox => (width - extras).max(0.0),
                BoxSizing::ContentBox => width,
            };
            (inner.min(content_available), true)
        }
        None => (total_max.min(content_available), false),
    };
    let target = match style.size.max_width {
        Length::Px(limit) => target.min(limit),
        _ => target,
    };
    let widths = distribute_columns(&columns.min, &columns.max, target, grows);
    let table_content_width: f32 = widths.iter().sum();
    let table_width = table_content_width + extras;

    let remaining = (outer_available - table_width).max(0.0);
    let offset = if metrics.is_margin_left_auto && metrics.is_margin_right_auto {
        metrics.margin.left + remaining / 2.0
    } else if metrics.is_margin_left_auto {
        metrics.margin.left + remaining
    } else {
        metrics.margin.left
    };
    let border_left = spec.x + offset;
    let content_x = border_left + metrics.border.left + metrics.padding.left;
    let mut cursor_y = spec.y + metrics.border.top + metrics.padding.top;

    let mut rows: Vec<LayoutBox> = Vec::new();
    for row in &model.rows {
        let laid = layout_row(
            context,
            row,
            &widths,
            content_x,
            cursor_y,
            table_content_width,
        );
        cursor_y = laid.bottom();
        rows.push(laid);
    }

    let content_height = cursor_y - (spec.y + metrics.border.top + metrics.padding.top);
    let rect = Rect::from_xywh(
        border_left,
        spec.y,
        table_width,
        content_height + metrics.vertical_extras(),
    );
    let mut layout_box = LayoutBox::plain(Some(node), BoxKind::Block, rect);
    layout_box.border_width = metrics.border;
    layout_box.border_color = style.border_color;
    layout_box.background = style.background_color;
    layout_box.radii = resolve_radii(style, rect.size.width, rect.size.height);
    layout_box.children = rows;
    BlockResult {
        layout_box,
        margin_bottom: metrics.margin.bottom,
    }
}

fn layout_row(
    context: &mut LayoutContext<'_>,
    row: &TableRow,
    widths: &[f32],
    x: f32,
    y: f32,
    row_width: f32,
) -> LayoutBox {
    let mut cells: Vec<LayoutBox> = Vec::new();
    let mut row_height = 0.0_f32;
    for cell in &row.cells {
        let end = (cell.column + cell.span).min(widths.len());
        let before: f32 = widths[..cell.column.min(widths.len())].iter().sum();
        let cell_width: f32 = widths[cell.column.min(widths.len())..end].iter().sum();
        let laid = layout_block_box(
            context,
            cell.node,
            BlockSpec {
                x: x + before,
                y,
                available_width: row_width,
                forced_width: Some(cell_width),
                forced_height: None,
            },
        );
        row_height = row_height.max(laid.layout_box.border_box.size.height);
        cells.push(laid.layout_box);
    }
    for cell in &mut cells {
        cell.border_box.size.height = row_height;
    }
    let mut row_box = LayoutBox::plain(
        Some(row.node),
        BoxKind::Block,
        Rect::from_xywh(x, y, row_width, row_height),
    );
    row_box.background = row.style.background_color;
    row_box.children = cells;
    row_box
}
