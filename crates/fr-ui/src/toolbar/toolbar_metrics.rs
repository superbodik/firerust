use fr_geom::geometry::rect::Rect;

use crate::tabs::tab_strip_metrics::TAB_STRIP_HEIGHT;
use crate::toolbar::toolbar_target::ToolbarTarget;

pub const TOOLBAR_HEIGHT: f32 = 46.0;
pub const CHROME_HEIGHT: f32 = TAB_STRIP_HEIGHT + TOOLBAR_HEIGHT;
const MARGIN: f32 = 10.0;
const BUTTON_SIZE: f32 = 34.0;
const BUTTON_GAP: f32 = 2.0;
const FIELD_GAP: f32 = 8.0;
const FIELD_HEIGHT: f32 = 34.0;
const ICON_BOX: f32 = 26.0;
const ICON_INSET: f32 = 5.0;

fn button_top() -> f32 {
    TAB_STRIP_HEIGHT + (TOOLBAR_HEIGHT - BUTTON_SIZE) / 2.0
}

pub fn button_rect(target: ToolbarTarget, toolbar_width: f32) -> Option<Rect> {
    let left = match target {
        ToolbarTarget::Back => MARGIN,
        ToolbarTarget::Forward => MARGIN + (BUTTON_SIZE + BUTTON_GAP),
        ToolbarTarget::Reload => MARGIN + 2.0 * (BUTTON_SIZE + BUTTON_GAP),
        ToolbarTarget::Menu => toolbar_width - MARGIN - BUTTON_SIZE,
        ToolbarTarget::Address | ToolbarTarget::Star | ToolbarTarget::Empty => return None,
    };
    Some(Rect::from_xywh(
        left,
        button_top(),
        BUTTON_SIZE,
        BUTTON_SIZE,
    ))
}

pub fn address_rect(toolbar_width: f32) -> Rect {
    let left = MARGIN + 3.0 * (BUTTON_SIZE + BUTTON_GAP) + FIELD_GAP - BUTTON_GAP;
    let right = toolbar_width - MARGIN - BUTTON_SIZE - FIELD_GAP;
    Rect::from_xywh(
        left,
        TAB_STRIP_HEIGHT + (TOOLBAR_HEIGHT - FIELD_HEIGHT) / 2.0,
        (right - left).max(0.0),
        FIELD_HEIGHT,
    )
}

pub fn security_rect(toolbar_width: f32) -> Rect {
    let field = address_rect(toolbar_width);
    Rect::from_xywh(
        field.origin.x + ICON_INSET,
        field.origin.y + (field.size.height - ICON_BOX) / 2.0,
        ICON_BOX,
        ICON_BOX,
    )
}

pub fn star_rect(toolbar_width: f32) -> Rect {
    let field = address_rect(toolbar_width);
    Rect::from_xywh(
        field.right() - ICON_INSET - ICON_BOX,
        field.origin.y + (field.size.height - ICON_BOX) / 2.0,
        ICON_BOX,
        ICON_BOX,
    )
}

pub fn address_text_rect(toolbar_width: f32) -> Rect {
    let field = address_rect(toolbar_width);
    let left = field.origin.x + ICON_INSET + ICON_BOX + 4.0;
    let right = field.right() - ICON_INSET - ICON_BOX - 4.0;
    Rect::from_xywh(
        left,
        field.origin.y,
        (right - left).max(0.0),
        field.size.height,
    )
}
