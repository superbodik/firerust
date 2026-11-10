use fr_geom::geometry::rect::Rect;

use crate::tabs::tab_strip_metrics::TAB_STRIP_HEIGHT;
use crate::toolbar::toolbar_target::ToolbarTarget;

pub const TOOLBAR_HEIGHT: f32 = 40.0;
pub const CHROME_HEIGHT: f32 = TAB_STRIP_HEIGHT + TOOLBAR_HEIGHT;
const MARGIN: f32 = 8.0;
const BUTTON_SIZE: f32 = 28.0;
const BUTTON_GAP: f32 = 4.0;
const FIELD_INSET: f32 = 6.0;

pub fn button_rect(target: ToolbarTarget) -> Option<Rect> {
    let slot = match target {
        ToolbarTarget::Back => 0.0,
        ToolbarTarget::Forward => 1.0,
        ToolbarTarget::Reload => 2.0,
        ToolbarTarget::Address | ToolbarTarget::Empty => return None,
    };
    Some(Rect::from_xywh(
        MARGIN + slot * (BUTTON_SIZE + BUTTON_GAP),
        TAB_STRIP_HEIGHT + (TOOLBAR_HEIGHT - BUTTON_SIZE) / 2.0,
        BUTTON_SIZE,
        BUTTON_SIZE,
    ))
}

pub fn address_rect(toolbar_width: f32) -> Rect {
    let left = MARGIN + 3.0 * (BUTTON_SIZE + BUTTON_GAP) + FIELD_INSET;
    Rect::from_xywh(
        left,
        TAB_STRIP_HEIGHT + FIELD_INSET,
        (toolbar_width - left - MARGIN).max(0.0),
        TOOLBAR_HEIGHT - 2.0 * FIELD_INSET,
    )
}
