use fr_geom::geometry::point::Point;

use crate::tabs::tab_strip_metrics::{TAB_STRIP_HEIGHT, close_rect, new_tab_rect, tab_rect};
use crate::tabs::tab_strip_target::TabStripTarget;

pub fn hit_tab_strip(point: Point, count: usize, strip_width: f32) -> Option<TabStripTarget> {
    if point.y < 0.0 || point.y >= TAB_STRIP_HEIGHT {
        return None;
    }
    for index in 0..count {
        if count > 1 && close_rect(index, count, strip_width).contains(point) {
            return Some(TabStripTarget::Close(index));
        }
        if tab_rect(index, count, strip_width).contains(point) {
            return Some(TabStripTarget::Tab(index));
        }
    }
    if new_tab_rect(count, strip_width).contains(point) {
        return Some(TabStripTarget::New);
    }
    Some(TabStripTarget::Empty)
}
