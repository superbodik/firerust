use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::point::Point;
use fr_text::engine::text_engine::TextEngine;

use crate::boxes::layout_box::LayoutBox;
use crate::hit_test::element_at::element_at;
use crate::hit_test::run_at::run_at;

pub fn event_target_at(root: &LayoutBox, engine: &mut TextEngine, point: Point) -> Option<NodeId> {
    run_at(root, engine, point).or_else(|| element_at(root, point))
}
