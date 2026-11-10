use crate::context::layout_context::LayoutContext;
use crate::intrinsic::outer_widths::outer_widths;
use crate::table::table_model::TableModel;

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnWidths {
    pub min: Vec<f32>,
    pub max: Vec<f32>,
}

pub fn measure_columns(context: &mut LayoutContext<'_>, model: &TableModel) -> ColumnWidths {
    let mut min = vec![0.0_f32; model.columns];
    let mut max = vec![0.0_f32; model.columns];

    for row in &model.rows {
        for cell in row.cells.iter().filter(|cell| cell.span == 1) {
            let widths = outer_widths(context, cell.node, &cell.style);
            min[cell.column] = min[cell.column].max(widths.min);
            max[cell.column] = max[cell.column].max(widths.max);
        }
    }
    for row in &model.rows {
        for cell in row.cells.iter().filter(|cell| cell.span > 1) {
            let widths = outer_widths(context, cell.node, &cell.style);
            let end = (cell.column + cell.span).min(model.columns);
            spread(&mut min[cell.column..end], widths.min);
            spread(&mut max[cell.column..end], widths.max);
        }
    }
    ColumnWidths { min, max }
}

fn spread(columns: &mut [f32], needed: f32) {
    if columns.is_empty() {
        return;
    }
    let have: f32 = columns.iter().sum();
    if needed <= have {
        return;
    }
    let extra = (needed - have) / columns.len() as f32;
    for column in columns {
        *column += extra;
    }
}
