use std::collections::HashMap;

use cosmic_text::{FontSystem, SwashCache};

use crate::render::content_widths::ContentWidths;
use crate::render::text_metrics::TextMetrics;

const CACHE_LIMIT: usize = 40_000;

pub struct TextEngine {
    font_system: FontSystem,
    swash_cache: SwashCache,
    metrics_cache: HashMap<u64, TextMetrics>,
    widths_cache: HashMap<u64, ContentWidths>,
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            metrics_cache: HashMap::new(),
            widths_cache: HashMap::new(),
        }
    }

    pub fn parts_mut(&mut self) -> (&mut FontSystem, &mut SwashCache) {
        (&mut self.font_system, &mut self.swash_cache)
    }

    pub fn cached_metrics(&self, key: u64) -> Option<TextMetrics> {
        self.metrics_cache.get(&key).copied()
    }

    pub fn remember_metrics(&mut self, key: u64, metrics: TextMetrics) {
        if self.metrics_cache.len() >= CACHE_LIMIT {
            self.metrics_cache.clear();
        }
        self.metrics_cache.insert(key, metrics);
    }

    pub fn cached_widths(&self, key: u64) -> Option<ContentWidths> {
        self.widths_cache.get(&key).copied()
    }

    pub fn remember_widths(&mut self, key: u64, widths: ContentWidths) {
        if self.widths_cache.len() >= CACHE_LIMIT {
            self.widths_cache.clear();
        }
        self.widths_cache.insert(key, widths);
    }
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}
