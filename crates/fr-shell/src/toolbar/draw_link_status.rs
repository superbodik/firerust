use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::draw::stroke_rounded_rect::stroke_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_theme::colors::palette::Palette;

use crate::toolbar::fit_text::fit_text;
use crate::toolbar::text_width::text_width;
use crate::toolbar::toolbar_run::toolbar_run;

const FONT_SIZE: f32 = 12.0;
const HEIGHT: f32 = 26.0;
const SIDE_PADDING: f32 = 10.0;
const MARGIN: f32 = 10.0;
const CORNER: f32 = 6.0;
const MAX_SHARE: f32 = 0.7;

pub fn draw_link_status(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    text: &str,
    lift: f32,
) {
    let canvas_width = canvas.width() as f32;
    let available = (canvas_width * MAX_SHARE - 2.0 * SIDE_PADDING).max(0.0);
    let shown = fit_text(engine, text, FONT_SIZE, available);
    let extent = text_width(engine, &shown, FONT_SIZE);
    let width = extent + 2.0 * SIDE_PADDING;
    let top = canvas.height() as f32 - HEIGHT - MARGIN - lift;
    let area = Rect::from_xywh(MARGIN, top, width, HEIGHT);
    fill_rounded_rect(canvas, area, CORNER, palette.toolbar_background);
    stroke_rounded_rect(canvas, area, CORNER, 1.0, palette.divider);
    let run = toolbar_run(&shown, FONT_SIZE, palette.field_text);
    let mut shaped = shape_runs(engine, &[run], extent + 8.0, TextAlignment::Left);
    let text_top = top + (HEIGHT - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(MARGIN + SIDE_PADDING, text_top));
}
