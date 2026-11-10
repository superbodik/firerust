use fr_text::engine::text_engine::TextEngine;

use crate::toolbar::text_width::text_width;

const ELLIPSIS: &str = "…";

pub fn fit_text(engine: &mut TextEngine, text: &str, font_size: f32, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if text_width(engine, text, font_size) <= max_width {
        return text.to_owned();
    }
    let characters: Vec<char> = text.chars().collect();
    let mut low = 0;
    let mut high = characters.len();
    while low < high {
        let middle = (low + high).div_ceil(2);
        let candidate: String = characters[..middle].iter().collect::<String>() + ELLIPSIS;
        if text_width(engine, &candidate, font_size) <= max_width {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    characters[..low].iter().collect::<String>() + ELLIPSIS
}
