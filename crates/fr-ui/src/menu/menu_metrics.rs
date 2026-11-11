use fr_geom::geometry::rect::Rect;

use crate::menu::menu_entries::menu_entries;
use crate::menu::menu_entry::MenuEntry;
use crate::menu::menu_kind::MenuKind;
use crate::toolbar::toolbar_metrics::CHROME_HEIGHT;

pub const MAIN_WIDTH: f32 = 300.0;
pub const START_WIDTH: f32 = 260.0;
pub const PADDING: f32 = 6.0;
pub const ROW_HEIGHT: f32 = 34.0;
pub const SEPARATOR_HEIGHT: f32 = 9.0;
pub const ZOOM_HEIGHT: f32 = 40.0;
pub const EDGE_GAP: f32 = 10.0;
pub const START_BUTTON_GAP: f32 = 16.0;
pub const START_BUTTON_HEIGHT: f32 = 36.0;

pub fn menu_width(kind: MenuKind) -> f32 {
    match kind {
        MenuKind::Main => MAIN_WIDTH,
        MenuKind::Start => START_WIDTH,
    }
}

pub fn entry_height(entry: MenuEntry) -> f32 {
    match entry {
        MenuEntry::Item(_) => ROW_HEIGHT,
        MenuEntry::Separator => SEPARATOR_HEIGHT,
        MenuEntry::Zoom => ZOOM_HEIGHT,
    }
}

pub fn menu_height(kind: MenuKind) -> f32 {
    2.0 * PADDING
        + menu_entries(kind)
            .iter()
            .map(|entry| entry_height(*entry))
            .sum::<f32>()
}

pub fn menu_rect(kind: MenuKind, viewport_width: f32, viewport_height: f32) -> Rect {
    let width = menu_width(kind);
    let height = menu_height(kind);
    match kind {
        MenuKind::Main => Rect::from_xywh(
            (viewport_width - width - EDGE_GAP).max(0.0),
            CHROME_HEIGHT - 2.0,
            width,
            height,
        ),
        MenuKind::Start => Rect::from_xywh(
            START_BUTTON_GAP,
            (viewport_height - START_BUTTON_GAP - START_BUTTON_HEIGHT - 8.0 - height).max(0.0),
            width,
            height,
        ),
    }
}
