use std::time::Duration;

use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_theme::colors::palette::Palette;
use fr_ui::toolbar::toolbar_metrics::CHROME_HEIGHT;

const BAR_HEIGHT: f32 = 3.0;
const SWEEP_SECONDS: f32 = 1.4;
const SEGMENT_SHARE: f32 = 0.3;

pub fn draw_load_bar(canvas: &mut Canvas, palette: &Palette, elapsed: Duration) {
    let width = canvas.width() as f32;
    let top = CHROME_HEIGHT - BAR_HEIGHT - 1.0;
    fill_rect(
        canvas,
        Rect::from_xywh(0.0, top, width, BAR_HEIGHT),
        palette.loading_track,
    );
    let phase = (elapsed.as_secs_f32() / SWEEP_SECONDS).fract();
    let segment = width * SEGMENT_SHARE;
    let left = phase * (width + segment) - segment;
    let start = left.max(0.0);
    let end = (left + segment).min(width);
    if end > start {
        fill_rounded_rect(
            canvas,
            Rect::from_xywh(start, top, end - start, BAR_HEIGHT),
            BAR_HEIGHT / 2.0,
            palette.accent,
        );
    }
}
