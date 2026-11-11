use fr_geom::geometry::rect::Rect;
use fr_raster::draw::draw_image::draw_image;
use fr_raster::draw::fill_border_ring::fill_border_ring;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::draw::fill_rounded_corners::fill_rounded_corners;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::draw_text::draw_text;

use crate::display::display_item::DisplayItem;

pub fn paint_item(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    item: &DisplayItem,
    destination: Rect,
    scale: f32,
) {
    match item {
        DisplayItem::Rect { color, .. } => fill_rect(canvas, destination, *color),
        DisplayItem::RoundedRect { radii, color, .. } => {
            fill_rounded_corners(canvas, destination, radii.scaled(scale), *color)
        }
        DisplayItem::Border {
            radii,
            widths,
            color,
            ..
        } => fill_border_ring(
            canvas,
            destination,
            radii.scaled(scale),
            widths.map(|width| width * scale),
            *color,
        ),
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
            let text_runs: Vec<_> = runs
                .iter()
                .map(|run| {
                    let mut text_run = run.as_text_run();
                    text_run.font_size *= scale;
                    text_run.line_height *= scale;
                    text_run
                })
                .collect();
            draw_text(
                engine,
                canvas,
                &text_runs,
                destination.size.width,
                *alignment,
                destination.origin,
            );
        }
    }
}
