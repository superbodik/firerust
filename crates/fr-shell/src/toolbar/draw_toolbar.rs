use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_ui::address::address_field::AddressField;
use fr_ui::toolbar::toolbar_metrics::{TOOLBAR_HEIGHT, address_rect, button_rect};
use fr_ui::toolbar::toolbar_target::ToolbarTarget;

use crate::toolbar::text_width::text_width;
use crate::toolbar::toolbar_run::toolbar_run;
use crate::toolbar::toolbar_state::ToolbarState;
use crate::toolbar::visible_slice::visible_slice;

const FIELD_FONT_SIZE: f32 = 14.0;
const BUTTON_FONT_SIZE: f32 = 18.0;
const FIELD_PADDING: f32 = 8.0;

pub fn draw_toolbar(canvas: &mut Canvas, engine: &mut TextEngine, state: &ToolbarState<'_>) {
    let width = canvas.width() as f32;
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, 0.0, width, TOOLBAR_HEIGHT),
        Rgba::opaque(236, 236, 241),
    );
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, TOOLBAR_HEIGHT - 1.0, width, 1.0),
        Rgba::opaque(200, 200, 208),
    );
    draw_button(canvas, engine, ToolbarTarget::Back, "←", state.can_go_back);
    draw_button(
        canvas,
        engine,
        ToolbarTarget::Forward,
        "→",
        state.can_go_forward,
    );
    draw_button(canvas, engine, ToolbarTarget::Reload, "↻", true);
    draw_field(canvas, engine, state.field, address_rect(width));
}

fn draw_button(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    target: ToolbarTarget,
    glyph: &str,
    is_enabled: bool,
) {
    let Some(rect) = button_rect(target) else {
        return;
    };
    let color = if is_enabled {
        Rgba::opaque(40, 40, 48)
    } else {
        Rgba::opaque(170, 170, 178)
    };
    let run = toolbar_run(glyph, BUTTON_FONT_SIZE, color);
    let mut shaped = shape_runs(engine, &[run], rect.size.width, TextAlignment::Center);
    let top = rect.origin.y + (rect.size.height - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(rect.origin.x, top));
}

fn draw_field(canvas: &mut Canvas, engine: &mut TextEngine, field: &AddressField, rect: Rect) {
    if rect.size.width <= 2.0 * FIELD_PADDING {
        return;
    }
    let border = if field.is_focused() {
        Rgba::opaque(11, 95, 165)
    } else {
        Rgba::opaque(190, 190, 198)
    };
    fill_rect(canvas, rect, border);
    fill_rect(
        canvas,
        Rect::from_xywh(
            rect.origin.x + 1.0,
            rect.origin.y + 1.0,
            rect.size.width - 2.0,
            rect.size.height - 2.0,
        ),
        Rgba::opaque(255, 255, 255),
    );
    let characters: Vec<char> = field.text().chars().collect();
    let text_width_limit = rect.size.width - 2.0 * FIELD_PADDING;
    let slice = visible_slice(
        engine,
        &characters,
        field.caret(),
        FIELD_FONT_SIZE,
        text_width_limit,
    );
    let shown: String = characters[slice.start..slice.end].iter().collect();
    let line_height = toolbar_run("", FIELD_FONT_SIZE, Rgba::BLACK).line_height;
    let left = rect.origin.x + FIELD_PADDING;
    let top = rect.origin.y + (rect.size.height - line_height) / 2.0;
    let shown_width = text_width(engine, &shown, FIELD_FONT_SIZE);
    if field.is_focused() && field.selects_all() {
        fill_rect(
            canvas,
            Rect::from_xywh(left, top, shown_width, line_height),
            Rgba::opaque(179, 212, 252),
        );
    }
    if !shown.is_empty() {
        let run = toolbar_run(&shown, FIELD_FONT_SIZE, Rgba::opaque(25, 25, 30));
        let mut shaped = shape_runs(engine, &[run], text_width_limit + 8.0, TextAlignment::Left);
        shaped.draw(engine, canvas, Point::new(left, top));
    }
    if field.is_focused() && !field.selects_all() {
        let before: String = characters[slice.start..field.caret().max(slice.start)]
            .iter()
            .collect();
        let caret_x = left + text_width(engine, &before, FIELD_FONT_SIZE);
        fill_rect(
            canvas,
            Rect::from_xywh(caret_x, top, 1.0, line_height),
            Rgba::opaque(25, 25, 30),
        );
    }
}
