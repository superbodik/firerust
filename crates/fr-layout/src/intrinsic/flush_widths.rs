use fr_text::render::content_widths::ContentWidths;
use fr_text::render::measure_runs::measure_runs;

use crate::boxes::layout_run::LayoutRun;
use crate::context::layout_context::LayoutContext;
use crate::inline::inline_buffer::InlineBuffer;
use crate::intrinsic::widen::widen;

pub fn flush_widths(
    context: &mut LayoutContext<'_>,
    buffer: &mut InlineBuffer,
    widths: &mut ContentWidths,
) {
    let runs = std::mem::take(buffer).into_runs();
    if runs.is_empty() {
        return;
    }
    let measured = {
        let text_runs: Vec<_> = runs.iter().map(LayoutRun::as_text_run).collect();
        measure_runs(context.text, &text_runs)
    };
    widen(widths, measured);
}
