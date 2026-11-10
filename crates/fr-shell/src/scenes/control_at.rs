use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::point::Point;
use fr_layout::control::control_kind_of::control_kind_of;
use fr_layout::hit_test::event_target_at::event_target_at;
use fr_tab::set::tab_set::TabSet;
use fr_text::engine::text_engine::TextEngine;

pub fn control_at(tabs: &mut TabSet, engine: &mut TextEngine, page_point: Point) -> Option<NodeId> {
    let content = tabs.active_mut().content_mut()?;
    let layout = content.layout.as_ref()?;
    let target = event_target_at(&layout.root, engine, page_point)?;
    let document = content.document.borrow();
    control_kind_of(&document, target).map(|_| target)
}
