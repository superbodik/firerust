use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::render::run_key::run_key;
use crate::render::text_alignment::TextAlignment;
use crate::render::text_run::TextRun;

pub fn paint_key(runs: &[TextRun<'_>], width: f32, alignment: TextAlignment) -> u64 {
    let mut hasher = DefaultHasher::new();
    run_key(runs, width, alignment).hash(&mut hasher);
    for run in runs {
        run.color.r.hash(&mut hasher);
        run.color.g.hash(&mut hasher);
        run.color.b.hash(&mut hasher);
        run.color.a.hash(&mut hasher);
        run.is_underlined.hash(&mut hasher);
        run.is_struck.hash(&mut hasher);
    }
    hasher.finish()
}
