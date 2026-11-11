use fr_geom::geometry::rect::Rect;

use crate::tabs::window_control::WindowControl;

pub const TAB_STRIP_HEIGHT: f32 = 40.0;
const MARGIN: f32 = 8.0;
const MIN_TAB_WIDTH: f32 = 80.0;
const MAX_TAB_WIDTH: f32 = 240.0;
const TAB_TOP: f32 = 6.0;
const NEW_BUTTON_SIZE: f32 = 30.0;
const CLOSE_SIZE: f32 = 20.0;
const CLOSE_INSET: f32 = 8.0;
pub const CONTROL_WIDTH: f32 = 46.0;
const DRAG_GAP: f32 = 40.0;

pub fn controls_width() -> f32 {
    CONTROL_WIDTH * WindowControl::ALL.len() as f32
}

pub fn control_rect(control: WindowControl, strip_width: f32) -> Rect {
    Rect::from_xywh(
        strip_width - controls_width() + control.slot() * CONTROL_WIDTH,
        0.0,
        CONTROL_WIDTH,
        TAB_STRIP_HEIGHT,
    )
}

fn tabs_area_width(strip_width: f32) -> f32 {
    (strip_width - MARGIN - NEW_BUTTON_SIZE - controls_width() - DRAG_GAP).max(0.0)
}

pub fn tab_width(count: usize, strip_width: f32) -> f32 {
    if count == 0 {
        return MAX_TAB_WIDTH;
    }
    (tabs_area_width(strip_width) / count as f32).clamp(MIN_TAB_WIDTH, MAX_TAB_WIDTH)
}

pub fn tab_rect(index: usize, count: usize, strip_width: f32) -> Rect {
    let width = tab_width(count, strip_width);
    Rect::from_xywh(
        MARGIN + index as f32 * width,
        TAB_TOP,
        width,
        TAB_STRIP_HEIGHT - TAB_TOP,
    )
}

pub fn close_rect(index: usize, count: usize, strip_width: f32) -> Rect {
    let tab = tab_rect(index, count, strip_width);
    Rect::from_xywh(
        tab.right() - CLOSE_SIZE - CLOSE_INSET,
        tab.origin.y + (tab.size.height - CLOSE_SIZE) / 2.0,
        CLOSE_SIZE,
        CLOSE_SIZE,
    )
}

pub fn new_tab_rect(count: usize, strip_width: f32) -> Rect {
    let last = tab_rect(count.saturating_sub(1), count.max(1), strip_width);
    let left = if count == 0 {
        MARGIN
    } else {
        last.right() + 4.0
    };
    Rect::from_xywh(
        left,
        TAB_TOP + (TAB_STRIP_HEIGHT - TAB_TOP - NEW_BUTTON_SIZE) / 2.0,
        NEW_BUTTON_SIZE,
        NEW_BUTTON_SIZE,
    )
}
