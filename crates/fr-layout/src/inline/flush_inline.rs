use fr_css::cascade::computed_style::ComputedStyle;
use fr_geom::geometry::rect::Rect;
use fr_text::render::measure_text::measure_text;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::boxes::layout_run::LayoutRun;
use crate::boxes::text_box::TextBox;
use crate::context::layout_context::LayoutContext;
use crate::flow::flow_state::FlowState;
use crate::inline::alignment_of::alignment_of;

pub fn flush_inline(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    container: &ComputedStyle,
) {
    let runs = std::mem::take(&mut state.inline).into_runs();
    if runs.is_empty() {
        return;
    }
    let alignment = alignment_of(container.text_align);
    let height = {
        let text_runs: Vec<_> = runs.iter().map(LayoutRun::as_text_run).collect();
        measure_text(context.text, &text_runs, state.area.width, alignment).height
    };
    if height <= 0.0 {
        return;
    }
    state.cursor += state.pending_margin;
    state.pending_margin = 0.0;
    let rect = Rect::from_xywh(state.area.x, state.cursor, state.area.width, height);
    state.boxes.push(LayoutBox::plain(
        None,
        BoxKind::Text(TextBox { runs, alignment }),
        rect,
    ));
    state.cursor += height;
    state.has_placed = true;
}
