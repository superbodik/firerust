use cosmic_text::Metrics;

pub const DEFAULT_FONT_SIZE: f32 = 16.0;
const MIN_FONT_SIZE: f32 = 1.0;

pub fn metrics_for(font_size: f32, line_height: f32) -> Metrics {
    let size = font_size.max(MIN_FONT_SIZE);
    Metrics::new(size, line_height.max(size))
}
