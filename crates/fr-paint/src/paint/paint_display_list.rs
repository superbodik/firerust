use fr_geom::geometry::rect::Rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;

use crate::display::display_list::DisplayList;
use crate::paint::paint_item::paint_item;

pub fn paint_display_list(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    list: &DisplayList,
    scroll_y: f32,
) {
    let viewport_height = canvas.height() as f32;
    for item in &list.items {
        let rect = item.rect();
        let top = rect.origin.y - scroll_y;
        if top > viewport_height || top + rect.size.height < 0.0 {
            continue;
        }
        let destination = Rect::from_xywh(rect.origin.x, top, rect.size.width, rect.size.height);
        paint_item(canvas, engine, item, destination);
    }
}
