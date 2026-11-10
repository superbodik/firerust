use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_geom::geometry::point::Point;
use fr_text::engine::text_engine::TextEngine;

use crate::boxes::layout_box::LayoutBox;
use crate::hit_test::run_at::run_at;
use crate::inline::link_of::link_of;

pub fn link_at(
    root: &LayoutBox,
    engine: &mut TextEngine,
    document: &Document,
    point: Point,
) -> Option<NodeId> {
    let owner = run_at(root, engine, point)?;
    link_of(document, owner)
}
