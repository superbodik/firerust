use fr_geom::geometry::point::Point;

use crate::hover::chrome_hover::ChromeHover;
use crate::tabs::hit_tab_strip::hit_tab_strip;
use crate::tabs::tab_strip_target::TabStripTarget;
use crate::toolbar::hit_toolbar::hit_toolbar;
use crate::toolbar::toolbar_target::ToolbarTarget;

pub fn hover_chrome(point: Point, tab_count: usize, width: f32) -> ChromeHover {
    if let Some(target) = hit_tab_strip(point, tab_count, width) {
        return match target {
            TabStripTarget::Tab(index) => ChromeHover::Tab(index),
            TabStripTarget::Close(index) => ChromeHover::TabClose(index),
            TabStripTarget::New => ChromeHover::NewTab,
            TabStripTarget::Control(control) => ChromeHover::Control(control),
            TabStripTarget::Empty => ChromeHover::Nothing,
        };
    }
    match hit_toolbar(point, width) {
        Some(ToolbarTarget::Empty) | None => ChromeHover::Nothing,
        Some(target) => ChromeHover::Toolbar(target),
    }
}
