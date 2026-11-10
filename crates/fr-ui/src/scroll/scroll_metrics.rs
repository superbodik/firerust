#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScrollMetrics {
    pub content_height: f32,
    pub viewport_height: f32,
}

impl ScrollMetrics {
    pub const fn new(content_height: f32, viewport_height: f32) -> Self {
        Self {
            content_height,
            viewport_height,
        }
    }

    pub fn max_offset(&self) -> f32 {
        (self.content_height - self.viewport_height).max(0.0)
    }
}
