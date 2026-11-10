use fr_geom::geometry::rect::Rect;

use crate::scroll::scroll_metrics::ScrollMetrics;

const THUMB_WIDTH: f32 = 8.0;
const THUMB_MARGIN: f32 = 2.0;
const MIN_THUMB_HEIGHT: f32 = 24.0;

pub fn scrollbar_thumb(offset: f32, metrics: ScrollMetrics, viewport_width: f32) -> Option<Rect> {
    if metrics.content_height <= metrics.viewport_height || metrics.viewport_height <= 0.0 {
        return None;
    }
    let track = metrics.viewport_height;
    let height = (track * track / metrics.content_height)
        .max(MIN_THUMB_HEIGHT)
        .min(track);
    let travel = track - height;
    let progress = (offset / metrics.max_offset()).clamp(0.0, 1.0);
    Some(Rect::from_xywh(
        viewport_width - THUMB_WIDTH - THUMB_MARGIN,
        travel * progress,
        THUMB_WIDTH,
        height,
    ))
}
