use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_icons::catalog::icon::Icon;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::draw::stroke_rounded_rect::stroke_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_theme::colors::palette::Palette;
use fr_ui::address::address_field::AddressField;
use fr_ui::hover::chrome_hover::ChromeHover;
use fr_ui::tabs::tab_strip_metrics::TAB_STRIP_HEIGHT;
use fr_ui::toolbar::toolbar_metrics::{
    CHROME_HEIGHT, TOOLBAR_HEIGHT, address_rect, address_text_rect, button_rect, security_rect,
    star_rect,
};
use fr_ui::toolbar::toolbar_target::ToolbarTarget;

use crate::toolbar::draw_icon::draw_icon;
use crate::toolbar::draw_icon_button::draw_icon_button;
use crate::toolbar::text_width::text_width;
use crate::toolbar::toolbar_run::toolbar_run;
use crate::toolbar::toolbar_state::ToolbarState;
use crate::toolbar::visible_slice::visible_slice;

const FIELD_FONT_SIZE: f32 = 14.0;
const BUTTON_ICON_SIZE: f32 = 16.0;
const FIELD_ICON_SIZE: f32 = 14.0;
const FOCUS_RING: f32 = 2.0;
const FIELD_CORNER: f32 = 8.0;
const HAIRLINE: f32 = 1.0;

pub fn draw_toolbar(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    state: &ToolbarState<'_>,
) {
    let width = canvas.width() as f32;
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, TAB_STRIP_HEIGHT, width, TOOLBAR_HEIGHT),
        palette.toolbar_background,
    );
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, CHROME_HEIGHT - 1.0, width, 1.0),
        palette.divider,
    );
    let hovered = |target: ToolbarTarget| state.hover == ChromeHover::Toolbar(target);
    let enabled = |is_enabled: bool| {
        if is_enabled {
            palette.icon
        } else {
            palette.icon_disabled
        }
    };
    let reload_icon = if state.is_loading {
        Icon::Close
    } else {
        Icon::ArrowClockwise
    };
    let buttons = [
        (
            ToolbarTarget::Back,
            Icon::ArrowLeft,
            enabled(state.can_go_back),
        ),
        (
            ToolbarTarget::Forward,
            Icon::ArrowRight,
            enabled(state.can_go_forward),
        ),
        (ToolbarTarget::Reload, reload_icon, palette.icon),
        (ToolbarTarget::Menu, Icon::MenuDots, palette.icon),
    ];
    for (target, icon, color) in buttons {
        let Some(area) = button_rect(target, width) else {
            continue;
        };
        let is_disabled = (target == ToolbarTarget::Back && !state.can_go_back)
            || (target == ToolbarTarget::Forward && !state.can_go_forward);
        draw_icon_button(
            canvas,
            palette,
            area,
            icon,
            BUTTON_ICON_SIZE,
            color,
            hovered(target) && !is_disabled,
        );
    }
    draw_field(canvas, engine, palette, state, width);
}

fn draw_field(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    state: &ToolbarState<'_>,
    width: f32,
) {
    let rect = address_rect(width);
    if rect.size.width < 80.0 {
        return;
    }
    let field = state.field;
    let radius = FIELD_CORNER;
    let is_hovered = matches!(
        state.hover,
        ChromeHover::Toolbar(ToolbarTarget::Address | ToolbarTarget::Star)
    );
    let background = if field.is_focused() {
        palette.field_focused
    } else if is_hovered {
        palette.field_hover
    } else {
        palette.field_background
    };
    fill_rounded_rect(canvas, rect, radius, background);
    if field.is_focused() {
        stroke_rounded_rect(canvas, rect, radius, FOCUS_RING, palette.accent);
    } else {
        stroke_rounded_rect(canvas, rect, radius, HAIRLINE, palette.field_border);
    }
    draw_icon(
        canvas,
        state.badge.icon(),
        security_rect(width),
        FIELD_ICON_SIZE,
        state.badge.color(palette),
    );
    let star = star_rect(width);
    let star_hovered = state.hover == ChromeHover::Toolbar(ToolbarTarget::Star);
    if star_hovered {
        fill_rounded_rect(canvas, star, 6.0, palette.button_hover);
    }
    let (star_icon, star_color) = if state.is_bookmarked {
        (Icon::StarFill, palette.star_on)
    } else {
        (Icon::Star, palette.icon)
    };
    draw_icon(canvas, star_icon, star, FIELD_ICON_SIZE, star_color);
    draw_text(canvas, engine, palette, field, width);
}

fn draw_text(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    field: &AddressField,
    width: f32,
) {
    let area = address_text_rect(width);
    if area.size.width <= 8.0 {
        return;
    }
    let characters: Vec<char> = field.text().chars().collect();
    let slice = visible_slice(
        engine,
        &characters,
        field.caret(),
        FIELD_FONT_SIZE,
        area.size.width,
    );
    let shown: String = characters[slice.start..slice.end].iter().collect();
    let line_height = toolbar_run("", FIELD_FONT_SIZE, palette.field_text).line_height;
    let left = area.origin.x;
    let top = area.origin.y + (area.size.height - line_height) / 2.0;
    let shown_width = text_width(engine, &shown, FIELD_FONT_SIZE);
    if field.is_focused() && field.selects_all() {
        fill_rounded_rect(
            canvas,
            Rect::from_xywh(left - 2.0, top - 1.0, shown_width + 4.0, line_height + 2.0),
            4.0,
            palette.selection,
        );
    }
    if shown.is_empty() {
        if !field.is_focused() {
            let run = toolbar_run(
                "Search or type an address",
                FIELD_FONT_SIZE,
                palette.field_hint,
            );
            let mut shaped = shape_runs(engine, &[run], area.size.width + 8.0, TextAlignment::Left);
            shaped.draw(engine, canvas, Point::new(left, top));
        }
    } else {
        let run = toolbar_run(&shown, FIELD_FONT_SIZE, palette.field_text);
        let mut shaped = shape_runs(engine, &[run], area.size.width + 8.0, TextAlignment::Left);
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
            palette.field_text,
        );
    }
}
