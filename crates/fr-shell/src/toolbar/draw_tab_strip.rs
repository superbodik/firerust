use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_icons::catalog::icon::Icon;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;
use fr_theme::colors::palette::Palette;
use fr_ui::hover::chrome_hover::ChromeHover;
use fr_ui::tabs::tab_strip_metrics::{
    TAB_STRIP_HEIGHT, close_rect, control_rect, new_tab_rect, tab_rect,
};
use fr_ui::tabs::window_control::WindowControl;

use crate::toolbar::draw_icon::draw_icon;
use crate::toolbar::fit_text::fit_text;
use crate::toolbar::tab_strip_state::TabStripState;
use crate::toolbar::toolbar_run::toolbar_run;

const LABEL_FONT_SIZE: f32 = 13.0;
const TAB_CORNER: f32 = 10.0;
const HOVER_CORNER: f32 = 8.0;
const TAB_GAP: f32 = 2.0;
const TAB_ICON_SIZE: f32 = 14.0;
const CLOSE_ICON_SIZE: f32 = 10.0;
const ICON_LEFT: f32 = 12.0;
const LABEL_LEFT: f32 = 34.0;
const CLOSE_RESERVE: f32 = 34.0;
const ACTIVE_EXTENSION: f32 = 6.0;

pub fn draw_tab_strip(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    state: &TabStripState,
) {
    let width = canvas.width() as f32;
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, 0.0, width, TAB_STRIP_HEIGHT),
        palette.strip_background,
    );
    let count = state.titles.len();
    for index in 0..count {
        draw_tab(canvas, engine, palette, state, index, count, width);
    }
    let plus = new_tab_rect(count, width);
    if state.hover == ChromeHover::NewTab {
        fill_rounded_rect(canvas, plus, HOVER_CORNER, palette.button_hover);
    }
    draw_icon(canvas, Icon::Plus, plus, 14.0, palette.icon);
    draw_window_controls(canvas, palette, state, width);
}

fn draw_window_controls(canvas: &mut Canvas, palette: &Palette, state: &TabStripState, width: f32) {
    for control in WindowControl::ALL {
        let area = control_rect(control, width);
        let hovered = state.hover == ChromeHover::Control(control);
        let is_close = control == WindowControl::Close;
        let mut color = palette.icon;
        if hovered {
            let background = if is_close {
                palette.close_hover
            } else {
                palette.button_hover
            };
            fill_rect(canvas, area, background);
            if is_close {
                color = palette.close_hover_icon;
            }
        }
        let icon = match control {
            WindowControl::Minimize => Icon::Minimize,
            WindowControl::Maximize if state.maximized => Icon::Restore,
            WindowControl::Maximize => Icon::Maximize,
            WindowControl::Close => Icon::Close,
        };
        draw_icon(canvas, icon, area, 12.0, color);
    }
}

fn draw_tab(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    state: &TabStripState,
    index: usize,
    count: usize,
    width: f32,
) {
    let slot = tab_rect(index, count, width);
    let rect = Rect::from_xywh(
        slot.origin.x + TAB_GAP / 2.0,
        slot.origin.y,
        slot.size.width - TAB_GAP,
        slot.size.height,
    );
    let is_active = index == state.active;
    let is_hovered = matches!(
        state.hover,
        ChromeHover::Tab(hovered) | ChromeHover::TabClose(hovered) if hovered == index
    );
    if is_active {
        fill_rounded_rect(
            canvas,
            Rect::from_xywh(
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height + ACTIVE_EXTENSION,
            ),
            TAB_CORNER,
            palette.tab_active,
        );
    } else if is_hovered {
        fill_rounded_rect(
            canvas,
            Rect::from_xywh(
                rect.origin.x,
                rect.origin.y + 2.0,
                rect.size.width,
                rect.size.height - 4.0,
            ),
            HOVER_CORNER,
            palette.tab_hover,
        );
    }
    let is_discarded = state.discarded.get(index).copied().unwrap_or(false);
    let text_color = if is_discarded || !is_active && !is_hovered {
        palette.tab_text_muted
    } else {
        palette.tab_text
    };
    let icon = state.icons.get(index).copied().unwrap_or(Icon::Globe);
    draw_icon(
        canvas,
        icon,
        Rect::from_xywh(
            rect.origin.x + ICON_LEFT - 2.0,
            rect.origin.y,
            TAB_ICON_SIZE + 4.0,
            rect.size.height,
        ),
        TAB_ICON_SIZE,
        text_color,
    );
    let reserve = if count > 1 { CLOSE_RESERVE } else { 12.0 };
    let label_width = rect.size.width - LABEL_LEFT - reserve;
    let label = fit_text(engine, &state.titles[index], LABEL_FONT_SIZE, label_width);
    let run = toolbar_run(&label, LABEL_FONT_SIZE, text_color);
    let mut shaped = shape_runs(
        engine,
        &[run],
        label_width.max(1.0) + 8.0,
        TextAlignment::Left,
    );
    let top = rect.origin.y + (rect.size.height - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(rect.origin.x + LABEL_LEFT, top));
    if count > 1 && (is_active || is_hovered) {
        let close = close_rect(index, count, width);
        if state.hover == ChromeHover::TabClose(index) {
            fill_rounded_rect(canvas, close, close.size.width / 2.0, palette.button_hover);
        }
        draw_icon(canvas, Icon::Close, close, CLOSE_ICON_SIZE, palette.icon);
    }
}
