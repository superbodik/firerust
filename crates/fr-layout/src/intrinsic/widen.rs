use fr_text::render::content_widths::ContentWidths;

pub fn widen(widths: &mut ContentWidths, other: ContentWidths) {
    widths.min = widths.min.max(other.min);
    widths.max = widths.max.max(other.max);
}
