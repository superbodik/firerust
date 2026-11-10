use fr_window::input::key_input::KeyInput;

use crate::scroll::scroll_metrics::ScrollMetrics;

const ARROW_STEP: f32 = 48.0;
const PAGE_FRACTION: f32 = 0.9;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScrollState {
    offset: f32,
}

impl ScrollState {
    pub const fn new() -> Self {
        Self { offset: 0.0 }
    }

    pub fn offset(&self) -> f32 {
        self.offset
    }

    pub fn scroll_to(&mut self, target: f32, metrics: ScrollMetrics) -> bool {
        let next = target.clamp(0.0, metrics.max_offset()).round();
        let changed = next != self.offset;
        self.offset = next;
        changed
    }

    pub fn scroll_by(&mut self, delta: f32, metrics: ScrollMetrics) -> bool {
        self.scroll_to(self.offset + delta, metrics)
    }

    pub fn clamp(&mut self, metrics: ScrollMetrics) -> bool {
        self.scroll_to(self.offset, metrics)
    }

    pub fn apply_key(&mut self, key: KeyInput, metrics: ScrollMetrics) -> bool {
        let page = metrics.viewport_height * PAGE_FRACTION;
        match key {
            KeyInput::ArrowUp => self.scroll_by(-ARROW_STEP, metrics),
            KeyInput::ArrowDown => self.scroll_by(ARROW_STEP, metrics),
            KeyInput::PageUp => self.scroll_by(-page, metrics),
            KeyInput::PageDown | KeyInput::Space => self.scroll_by(page, metrics),
            KeyInput::Home => self.scroll_to(0.0, metrics),
            KeyInput::End => self.scroll_to(metrics.max_offset(), metrics),
            _ => false,
        }
    }
}
