use cosmic_text::{Buffer, Color as GlyphColor};
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;

use crate::engine::text_engine::TextEngine;

pub struct ShapedText {
    buffer: Buffer,
}

impl ShapedText {
    pub(crate) fn new(buffer: Buffer) -> Self {
        Self { buffer }
    }

    pub fn height(&self) -> f32 {
        self.buffer
            .layout_runs()
            .map(|layout| layout.line_height)
            .sum()
    }

    pub fn draw(&mut self, engine: &mut TextEngine, canvas: &mut Canvas, origin: Point) {
        let (font_system, swash_cache) = engine.parts_mut();
        self.buffer.draw(
            font_system,
            swash_cache,
            GlyphColor::rgb(0, 0, 0),
            |x, y, width, height, color| {
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
            },
        );
    }
}
