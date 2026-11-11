use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::render::text_alignment::TextAlignment;
use crate::render::text_run::TextRun;

pub fn run_key(runs: &[TextRun<'_>], width: f32, alignment: TextAlignment) -> u64 {
    let mut hasher = DefaultHasher::new();
    width.to_bits().hash(&mut hasher);
    alignment.hash(&mut hasher);
    for run in runs {
        run.text.hash(&mut hasher);
        run.font_size.to_bits().hash(&mut hasher);
        run.line_height.to_bits().hash(&mut hasher);
        run.font_weight.hash(&mut hasher);
        run.is_italic.hash(&mut hasher);
        run.family.hash(&mut hasher);
    }
    hasher.finish()
}
