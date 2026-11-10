use fr_geom::geometry::rect::Rect;
use fr_layout::boxes::layout_box::LayoutBox;

pub fn border_rects(layout_box: &LayoutBox) -> Vec<Rect> {
    let outer = layout_box.border_box;
    let widths = layout_box.border_width;
    let left = outer.origin.x;
    let top = outer.origin.y;
    let width = outer.size.width;
    let height = outer.size.height;
    let middle_height = (height - widths.top - widths.bottom).max(0.0);

    let mut rects: Vec<Rect> = Vec::new();
    if widths.top > 0.0 {
        rects.push(Rect::from_xywh(left, top, width, widths.top));
    }
    if widths.bottom > 0.0 {
        rects.push(Rect::from_xywh(
            left,
            top + height - widths.bottom,
            width,
            widths.bottom,
        ));
    }
    if widths.left > 0.0 {
        rects.push(Rect::from_xywh(
            left,
            top + widths.top,
            widths.left,
            middle_height,
        ));
    }
    if widths.right > 0.0 {
        rects.push(Rect::from_xywh(
            left + width - widths.right,
            top + widths.top,
            widths.right,
            middle_height,
        ));
    }
    rects
}
