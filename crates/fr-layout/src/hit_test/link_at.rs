use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::point::Point;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::boxes::layout_run::LayoutRun;

pub fn link_at(root: &LayoutBox, engine: &mut TextEngine, point: Point) -> Option<NodeId> {
    let mut current = root;
    while let Some(child) = current
        .children
        .iter()
        .rev()
        .find(|child| child.border_box.contains(point))
    {
        current = child;
    }
    let BoxKind::Text(text) = &current.kind else {
        return None;
    };
    let text_runs: Vec<_> = text.runs.iter().map(LayoutRun::as_text_run).collect();
    let shaped = shape_runs(
        engine,
        &text_runs,
        current.border_box.size.width,
        text.alignment,
    );
    let offset = shaped.offset_at(
        point.x - current.border_box.origin.x,
        point.y - current.border_box.origin.y,
    )?;
    let mut end = 0;
    for run in &text.runs {
        end += run.text.len();
        if offset < end {
            return run.link;
        }
    }
    None
}
