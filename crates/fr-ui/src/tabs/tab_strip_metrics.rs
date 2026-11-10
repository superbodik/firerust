use fr_geom::geometry::rect::Rect;

pub const TAB_STRIP_HEIGHT: f32 = 34.0;
const MARGIN: f32 = 8.0;
const MIN_TAB_WIDTH: f32 = 70.0;
const MAX_TAB_WIDTH: f32 = 220.0;
const TAB_TOP: f32 = 4.0;
const NEW_BUTTON_SIZE: f32 = 26.0;
const CLOSE_SIZE: f32 = 16.0;
const CLOSE_INSET: f32 = 6.0;

fn tabs_area_width(strip_width: f32) -> f32 {
    (strip_width - 2.0 * MARGIN - NEW_BUTTON_SIZE).max(0.0)
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
