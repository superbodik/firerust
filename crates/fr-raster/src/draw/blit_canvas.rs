use tiny_skia::{PixmapPaint, Transform};

use crate::surface::canvas::Canvas;

pub fn blit_canvas(target: &mut Canvas, source: &Canvas, x: i32, y: i32) {
    target.pixmap_mut().draw_pixmap(
        x,
        y,
        source.pixmap().as_ref(),
        &PixmapPaint::default(),
        Transform::identity(),
        None,
    );
}
