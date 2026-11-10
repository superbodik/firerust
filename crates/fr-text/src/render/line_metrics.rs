use cosmic_text::Metrics;

pub const DEFAULT_FONT_SIZE: f32 = 16.0;
pub const LINE_HEIGHT_FACTOR: f32 = 1.4;
const MIN_FONT_SIZE: f32 = 1.0;

pub fn metrics_for(font_size: f32) -> Metrics {
    let size = font_size.max(MIN_FONT_SIZE);
    Metrics::new(size, size * LINE_HEIGHT_FACTOR)
}
