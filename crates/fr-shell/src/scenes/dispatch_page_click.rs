use fr_geom::geometry::point::Point;
use fr_layout::hit_test::event_target_at::event_target_at;
use fr_tab::set::tab_set::TabSet;
use fr_text::engine::text_engine::TextEngine;

pub fn dispatch_page_click(
    tabs: &mut TabSet,
    engine: &mut TextEngine,
    page_point: Point,
    client: (f32, f32),
) -> bool {
    let Some(content) = tabs.active_mut().content_mut() else {
        return true;
    };
    if content.runtime.is_none() {
        return true;
    }
    let Some(layout) = content.layout.as_ref() else {
        return true;
    };
    let Some(node) = event_target_at(&layout.root, engine, page_point) else {
        return true;
    };
    content.dispatch_event(node, "click", Some(client))
}
