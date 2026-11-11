use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_theme::colors::palette::Palette;

use crate::toolbar::fit_text::fit_text;
use crate::toolbar::text_width::text_width;
use crate::toolbar::toolbar_run::toolbar_run;

const FONT_SIZE: f32 = 13.0;
const HEIGHT: f32 = 36.0;
const SIDE_PADDING: f32 = 18.0;
const BOTTOM_GAP: f32 = 22.0;
const CORNER: f32 = 8.0;
const SCREEN_MARGIN: f32 = 24.0;

pub fn draw_notice(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    message: &str,
    lift: f32,
) {
    let canvas_width = canvas.width() as f32;
    let available = (canvas_width - 2.0 * SCREEN_MARGIN - 2.0 * SIDE_PADDING).max(0.0);
    let shown = fit_text(engine, message, FONT_SIZE, available);
    let text_extent = text_width(engine, &shown, FONT_SIZE);
    let width = text_extent + 2.0 * SIDE_PADDING;
    let left = (canvas_width - width) / 2.0;
    let top = canvas.height() as f32 - HEIGHT - BOTTOM_GAP - lift;
    fill_rounded_rect(
        canvas,
        Rect::from_xywh(left, top, width, HEIGHT),
        CORNER,
        palette.notice_background,
    );
    let run = toolbar_run(&shown, FONT_SIZE, palette.notice_text);
    let mut shaped = shape_runs(engine, &[run], text_extent + 8.0, TextAlignment::Left);
    let text_top = top + (HEIGHT - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(left + SIDE_PADDING, text_top));
}
