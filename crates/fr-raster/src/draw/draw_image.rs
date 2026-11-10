use fr_geom::geometry::rect::Rect;
use tiny_skia::{FilterQuality, PixmapPaint, PixmapRef, Transform};

use crate::surface::canvas::Canvas;

pub fn draw_image(
    canvas: &mut Canvas,
    destination: Rect,
    width: u32,
    height: u32,
    premultiplied_rgba: &[u8],
) {
    let Some(source) = PixmapRef::from_bytes(premultiplied_rgba, width, height) else {
        return;
    };
    if destination.size.width <= 0.0 || destination.size.height <= 0.0 {
        return;
    }
    let scale_x = destination.size.width / width as f32;
    let scale_y = destination.size.height / height as f32;
    let transform = Transform::from_row(
        scale_x,
        0.0,
        0.0,
        scale_y,
        destination.origin.x,
        destination.origin.y,
    );
    let paint = PixmapPaint {
        quality: FilterQuality::Bilinear,
        ..PixmapPaint::default()
    };
    canvas
        .pixmap_mut()
        .draw_pixmap(0, 0, source, &paint, transform, None);
}
