use fr_geom::geometry::point::Point;

use crate::tabs::tab_strip_metrics::TAB_STRIP_HEIGHT;
use crate::toolbar::toolbar_metrics::{CHROME_HEIGHT, address_rect, button_rect, star_rect};
use crate::toolbar::toolbar_target::ToolbarTarget;

pub fn hit_toolbar(point: Point, toolbar_width: f32) -> Option<ToolbarTarget> {
    if point.y < TAB_STRIP_HEIGHT || point.y >= CHROME_HEIGHT {
        return None;
    }
    let buttons = [
        ToolbarTarget::Back,
        ToolbarTarget::Forward,
        ToolbarTarget::Reload,
        ToolbarTarget::Menu,
    ];
    for target in buttons {
        if button_rect(target, toolbar_width).is_some_and(|rect| rect.contains(point)) {
            return Some(target);
        }
    }
    if address_rect(toolbar_width).size.width > 0.0 && star_rect(toolbar_width).contains(point) {
        return Some(ToolbarTarget::Star);
    }
    if address_rect(toolbar_width).contains(point) {
        return Some(ToolbarTarget::Address);
    }
    Some(ToolbarTarget::Empty)
}
