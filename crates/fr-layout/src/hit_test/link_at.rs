use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_geom::geometry::point::Point;
use fr_text::engine::text_engine::TextEngine;

use crate::boxes::layout_box::LayoutBox;
use crate::hit_test::element_at::element_at;
use crate::hit_test::run_at::run_at;
use crate::inline::link_of::link_of;

pub fn link_at(
    root: &LayoutBox,
    engine: &mut TextEngine,
    document: &Document,
    point: Point,
) -> Option<NodeId> {
    if let Some(owner) = run_at(root, engine, point) {
        return link_of(document, owner);
    }
    let element = element_at(root, point)?;
    link_of(document, element)
}
