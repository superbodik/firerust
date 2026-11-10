use fr_geom::geometry::rect::Rect;
use fr_raster::draw::draw_image::draw_image;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;

use crate::display::display_item::DisplayItem;

pub fn paint_item(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    item: &DisplayItem,
    destination: Rect,
) {
    match item {
        DisplayItem::Rect { color, .. } => fill_rect(canvas, destination, *color),
        DisplayItem::Image { image, .. } => draw_image(
            canvas,
            destination,
            image.width(),
            image.height(),
            image.premultiplied_rgba(),
        ),
        DisplayItem::Text {
            alignment, runs, ..
        } => {
            let text_runs: Vec<_> = runs.iter().map(|run| run.as_text_run()).collect();
            let mut shaped = shape_runs(engine, &text_runs, destination.size.width, *alignment);
            shaped.draw(engine, canvas, destination.origin);
        }
    }
}
