use fr_geom::color::rgba::Rgba;
use fr_text::render::text_family::TextFamily;
use fr_text::render::text_run::TextRun;

pub fn toolbar_run(text: &str, font_size: f32, color: Rgba) -> TextRun<'_> {
    TextRun {
        text,
        color,
        font_size,
        line_height: (font_size * 1.3).ceil(),
        font_weight: 400,
        is_italic: false,
        family: TextFamily::SansSerif,
        is_underlined: false,
        is_struck: false,
    }
}
