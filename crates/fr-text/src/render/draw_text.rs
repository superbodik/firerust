use cosmic_text::{Attrs, Buffer, Color as GlyphColor, Family, Metrics, Shaping};
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;

use crate::engine::text_engine::TextEngine;
use crate::render::text_style::TextStyle;

pub fn draw_text(
    engine: &mut TextEngine,
    canvas: &mut Canvas,
    origin: Point,
    text: &str,
    style: TextStyle,
) -> f32 {
    let max_width = canvas.width() as f32 - origin.x;
    let (font_system, swash_cache) = engine.parts_mut();

    let mut buffer = Buffer::new(
        font_system,
        Metrics::new(style.font_size, style.line_height),
    );
    let mut buffer = buffer.borrow_with(font_system);
    buffer.set_size(Some(max_width), None);
    buffer.set_text(
        text,
        &Attrs::new().family(Family::SansSerif),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(false);
    let line_count = buffer.layout_runs().count().max(1);

    let glyph_color = GlyphColor::rgba(style.color.r, style.color.g, style.color.b, style.color.a);
    buffer.draw(swash_cache, glyph_color, |x, y, width, height, color| {
        let pixel = Rect::from_xywh(
            origin.x + x as f32,
            origin.y + y as f32,
            width as f32,
            height as f32,
        );
        fill_rect(
            canvas,
            pixel,
            Rgba::new(color.r(), color.g(), color.b(), color.a()),
        );
    });

    line_count as f32 * style.line_height
}
