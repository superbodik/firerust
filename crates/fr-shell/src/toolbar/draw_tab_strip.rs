use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_ui::tabs::tab_strip_metrics::{TAB_STRIP_HEIGHT, close_rect, new_tab_rect, tab_rect};

use crate::toolbar::fit_text::fit_text;
use crate::toolbar::tab_strip_state::TabStripState;
use crate::toolbar::toolbar_run::toolbar_run;

const LABEL_FONT_SIZE: f32 = 13.0;
const GLYPH_FONT_SIZE: f32 = 16.0;
const LABEL_PADDING: f32 = 10.0;
const CLOSE_RESERVE: f32 = 28.0;

pub fn draw_tab_strip(canvas: &mut Canvas, engine: &mut TextEngine, state: &TabStripState) {
    let width = canvas.width() as f32;
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, 0.0, width, TAB_STRIP_HEIGHT),
        Rgba::opaque(214, 214, 222),
    );
    let count = state.titles.len();
    for index in 0..count {
        draw_tab(canvas, engine, state, index, count, width);
    }
    let plus = new_tab_rect(count, width);
    draw_centered(
        canvas,
        engine,
        plus,
        "+",
        GLYPH_FONT_SIZE,
        Rgba::opaque(40, 40, 48),
    );
}

fn draw_tab(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    state: &TabStripState,
    index: usize,
    count: usize,
    width: f32,
) {
    let rect = tab_rect(index, count, width);
    let is_active = index == state.active;
    let background = if is_active {
        Rgba::opaque(236, 236, 241)
    } else {
        Rgba::opaque(198, 198, 208)
    };
    fill_rect(
        canvas,
        Rect::from_xywh(
            rect.origin.x + 1.0,
            rect.origin.y,
            rect.size.width - 2.0,
            rect.size.height,
        ),
        background,
    );
    let is_discarded = state.discarded.get(index).copied().unwrap_or(false);
    let color = if is_discarded {
        Rgba::opaque(120, 120, 130)
    } else {
        Rgba::opaque(30, 30, 38)
    };
    let label_width = rect.size.width - LABEL_PADDING - CLOSE_RESERVE;
    let label = fit_text(engine, &state.titles[index], LABEL_FONT_SIZE, label_width);
    let run = toolbar_run(&label, LABEL_FONT_SIZE, color);
    let mut shaped = shape_runs(
        engine,
        &[run],
        label_width.max(1.0) + 8.0,
        TextAlignment::Left,
    );
    let top = rect.origin.y + (rect.size.height - shaped.height()) / 2.0;
    shaped.draw(
        engine,
        canvas,
        Point::new(rect.origin.x + LABEL_PADDING, top),
    );
    if count > 1 {
        let close = close_rect(index, count, width);
        draw_centered(canvas, engine, close, "×", GLYPH_FONT_SIZE, color);
    }
}

fn draw_centered(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    rect: Rect,
    glyph: &str,
    font_size: f32,
    color: Rgba,
) {
    let run = toolbar_run(glyph, font_size, color);
    let mut shaped = shape_runs(engine, &[run], rect.size.width, TextAlignment::Center);
    let top = rect.origin.y + (rect.size.height - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(rect.origin.x, top));
}
