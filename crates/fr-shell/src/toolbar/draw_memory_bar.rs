use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_theme::colors::palette::Palette;

use crate::toolbar::fit_text::fit_text;
use crate::toolbar::toolbar_run::toolbar_run;

pub const MEMORY_BAR_HEIGHT: f32 = 24.0;
const FONT_SIZE: f32 = 12.0;
const PADDING: f32 = 12.0;

pub fn draw_memory_bar(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    summary: &str,
) {
    let width = canvas.width() as f32;
    let top = canvas.height() as f32 - MEMORY_BAR_HEIGHT;
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, top, width, MEMORY_BAR_HEIGHT),
        palette.notice_background,
    );
    let text_width = width - 2.0 * PADDING;
    let shown = fit_text(engine, summary, FONT_SIZE, text_width);
    let run = toolbar_run(&shown, FONT_SIZE, palette.notice_text);
    let mut shaped = shape_runs(
        engine,
        &[run],
        text_width.max(1.0) + 8.0,
        TextAlignment::Left,
    );
    let text_top = top + (MEMORY_BAR_HEIGHT - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(PADDING, text_top));
}
