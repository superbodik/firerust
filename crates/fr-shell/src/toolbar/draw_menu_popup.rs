use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use fr_icons::catalog::icon::Icon;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::draw::stroke_rounded_rect::stroke_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_theme::colors::palette::Palette;
use fr_ui::menu::entry_rects::entry_rects;
use fr_ui::menu::menu_entry::MenuEntry;
use fr_ui::menu::menu_kind::MenuKind;
use fr_ui::menu::menu_metrics::menu_rect;
use fr_ui::menu::menu_target::MenuTarget;
use fr_ui::menu::zoom_in_rect::zoom_in_rect;
use fr_ui::menu::zoom_out_rect::zoom_out_rect;
use fr_ui::menu::zoom_value_rect::zoom_value_rect;

use crate::toolbar::draw_icon::draw_icon;
use crate::toolbar::draw_menu_text::draw_menu_text;
use crate::toolbar::menu_label::menu_label;
use crate::toolbar::text_width::text_width;

const CORNER: f32 = 10.0;
const ROW_CORNER: f32 = 6.0;
const FONT_SIZE: f32 = 14.0;
const SHORTCUT_SIZE: f32 = 12.0;
const ICON_SIZE: f32 = 16.0;
const LABEL_LEFT: f32 = 42.0;

pub struct MenuView {
    pub kind: MenuKind,
    pub hover: Option<MenuTarget>,
    pub zoom_percent: u32,
}

pub fn draw_menu_popup(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    view: &MenuView,
) {
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;
    let popup = menu_rect(view.kind, width, height);
    for (shift, alpha) in [(4.0, 26), (2.0, 34)] {
        let shadow = Rect::from_xywh(
            popup.origin.x,
            popup.origin.y + shift,
            popup.size.width,
            popup.size.height,
        );
        fill_rounded_rect(canvas, shadow, CORNER, Rgba::new(0, 0, 0, alpha));
    }
    fill_rounded_rect(canvas, popup, CORNER, palette.page_card);
    stroke_rounded_rect(canvas, popup, CORNER, 1.0, palette.page_border);
    for (entry, rect) in entry_rects(view.kind, width, height) {
        match entry {
            MenuEntry::Item(target) => draw_item(canvas, engine, palette, view, target, rect),
            MenuEntry::Separator => fill_rect(
                canvas,
                Rect::from_xywh(
                    rect.origin.x + 4.0,
                    rect.origin.y + 4.0,
                    rect.size.width - 8.0,
                    1.0,
                ),
                palette.page_border,
            ),
            MenuEntry::Zoom => draw_zoom(canvas, engine, palette, view, rect),
        }
    }
}

fn draw_item(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    view: &MenuView,
    target: MenuTarget,
    rect: Rect,
) {
    if view.hover == Some(target) {
        fill_rounded_rect(canvas, rect, ROW_CORNER, palette.button_hover);
    }
    let label = menu_label(target);
    let icon_area = Rect::from_xywh(rect.origin.x + 6.0, rect.origin.y, 28.0, rect.size.height);
    draw_icon(canvas, label.icon, icon_area, ICON_SIZE, palette.icon);
    draw_menu_text(
        canvas,
        engine,
        label.text,
        FONT_SIZE,
        palette.field_text,
        rect.origin.x + LABEL_LEFT,
        rect,
    );
    if !label.shortcut.is_empty() {
        let extent = text_width(engine, label.shortcut, SHORTCUT_SIZE);
        draw_menu_text(
            canvas,
            engine,
            label.shortcut,
            SHORTCUT_SIZE,
            palette.tab_text_muted,
            rect.right() - 12.0 - extent,
            rect,
        );
    }
}

fn draw_zoom(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    view: &MenuView,
    row: Rect,
) {
    let icon_area = Rect::from_xywh(row.origin.x + 6.0, row.origin.y, 28.0, row.size.height);
    draw_icon(canvas, Icon::ZoomIn, icon_area, ICON_SIZE, palette.icon);
    draw_menu_text(
        canvas,
        engine,
        "Zoom",
        FONT_SIZE,
        palette.field_text,
        row.origin.x + LABEL_LEFT,
        row,
    );
    let out = zoom_out_rect(row);
    let value = zoom_value_rect(row);
    let plus = zoom_in_rect(row);
    for (area, target) in [
        (out, MenuTarget::ZoomOut),
        (value, MenuTarget::ZoomReset),
        (plus, MenuTarget::ZoomIn),
    ] {
        if view.hover == Some(target) {
            fill_rounded_rect(canvas, area, ROW_CORNER, palette.button_hover);
        }
    }
    draw_icon(canvas, Icon::Minimize, out, 12.0, palette.icon);
    draw_icon(canvas, Icon::Plus, plus, 12.0, palette.icon);
    let percent = format!("{}%", view.zoom_percent);
    let extent = text_width(engine, &percent, FONT_SIZE);
    draw_menu_text(
        canvas,
        engine,
        &percent,
        FONT_SIZE,
        palette.field_text,
        value.origin.x + (value.size.width - extent) / 2.0,
        value,
    );
}
