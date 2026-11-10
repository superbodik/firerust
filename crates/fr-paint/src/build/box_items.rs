use fr_layout::boxes::box_kind::BoxKind;
use fr_layout::boxes::layout_box::LayoutBox;

use crate::build::border_rects::border_rects;
use crate::display::display_item::DisplayItem;

pub fn box_items(layout_box: &LayoutBox, items: &mut Vec<DisplayItem>) {
    if layout_box.background.a > 0 {
        items.push(DisplayItem::Rect {
            rect: layout_box.border_box,
            color: layout_box.background,
        });
    }
    for rect in border_rects(layout_box) {
        items.push(DisplayItem::Rect {
            rect,
            color: layout_box.border_color,
        });
    }
    match &layout_box.kind {
        BoxKind::Block => {}
        BoxKind::Text(text) | BoxKind::Marker(text) => items.push(DisplayItem::Text {
            rect: layout_box.border_box,
            alignment: text.alignment,
            runs: text.runs.clone(),
        }),
        BoxKind::Image(image) => items.push(DisplayItem::Image {
            rect: layout_box.border_box,
            image: image.clone(),
        }),
    }
}
