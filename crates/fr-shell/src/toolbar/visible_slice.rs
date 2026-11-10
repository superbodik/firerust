use fr_text::engine::text_engine::TextEngine;

use crate::toolbar::text_width::text_width;

pub struct VisibleSlice {
    pub start: usize,
    pub end: usize,
}

pub fn visible_slice(
    engine: &mut TextEngine,
    characters: &[char],
    caret: usize,
    font_size: f32,
    max_width: f32,
) -> VisibleSlice {
    let width_of = |engine: &mut TextEngine, from: usize, to: usize| {
        let piece: String = characters[from..to].iter().collect();
        text_width(engine, &piece, font_size)
    };
    let caret = caret.min(characters.len());
    let mut low = 0;
    let mut high = caret;
    while low < high {
        let middle = (low + high) / 2;
        if width_of(engine, middle, caret) > max_width {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    let start = low;
    let mut low = caret;
    let mut high = characters.len();
    while low < high {
        let middle = (low + high).div_ceil(2);
        if width_of(engine, start, middle) <= max_width {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    VisibleSlice { start, end: low }
}
