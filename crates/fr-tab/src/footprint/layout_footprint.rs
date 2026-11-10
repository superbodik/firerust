use fr_layout::boxes::box_kind::BoxKind;
use fr_layout::boxes::layout_box::LayoutBox;
use fr_paint::display::display_item::DisplayItem;

use crate::model::layout_cache::LayoutCache;

pub fn layout_footprint(layout: &LayoutCache) -> usize {
    let mut total = layout.list.items.len() * std::mem::size_of::<DisplayItem>();
    let mut pending: Vec<&LayoutBox> = vec![&layout.root];
    while let Some(current) = pending.pop() {
        total += std::mem::size_of::<LayoutBox>();
        if let BoxKind::Text(text) | BoxKind::Marker(text) = &current.kind {
            total += text.plain_text().len();
        }
        pending.extend(current.children.iter());
    }
    total
}
