use fr_layout::boxes::box_kind::BoxKind;
use fr_layout::boxes::layout_box::LayoutBox;

use crate::build::border_rects::border_rects;
use crate::display::display_item::DisplayItem;

pub fn box_items(layout_box: &LayoutBox, items: &mut Vec<DisplayItem>) {
    push_background(layout_box, items);
    push_border(layout_box, items);
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

fn push_background(layout_box: &LayoutBox, items: &mut Vec<DisplayItem>) {
    if layout_box.background.a == 0 {
        return;
    }
    if layout_box.radii.is_square() {
        items.push(DisplayItem::Rect {
            rect: layout_box.border_box,
            color: layout_box.background,
        });
        return;
    }
    items.push(DisplayItem::RoundedRect {
        rect: layout_box.border_box,
        radii: layout_box.radii,
        color: layout_box.background,
    });
}

fn push_border(layout_box: &LayoutBox, items: &mut Vec<DisplayItem>) {
    let widths = layout_box.border_width;
    if layout_box.radii.is_square() {
        for rect in border_rects(layout_box) {
            items.push(DisplayItem::Rect {
                rect,
                color: layout_box.border_color,
            });
        }
        return;
    }
    if widths.top <= 0.0 && widths.right <= 0.0 && widths.bottom <= 0.0 && widths.left <= 0.0 {
        return;
    }
    items.push(DisplayItem::Border {
        rect: layout_box.border_box,
        radii: layout_box.radii,
        widths: [widths.top, widths.right, widths.bottom, widths.left],
        color: layout_box.border_color,
    });
}
