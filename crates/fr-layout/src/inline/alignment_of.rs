use fr_css::value::text_align::TextAlign;
use fr_text::render::text_alignment::TextAlignment;

pub fn alignment_of(align: TextAlign) -> TextAlignment {
    match align {
        TextAlign::Left => TextAlignment::Left,
        TextAlign::Center => TextAlignment::Center,
        TextAlign::Right => TextAlignment::Right,
    }
}
