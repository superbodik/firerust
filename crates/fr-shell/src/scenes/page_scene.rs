use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_raster::draw::clear::clear;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::draw_text::draw_text;
use fr_text::render::text_style::TextStyle;
use fr_window::scene::render_scene::RenderScene;

const MARGIN: f32 = 24.0;

const STYLE: TextStyle = TextStyle {
    font_size: 16.0,
    line_height: 22.0,
    color: Rgba::BLACK,
};

pub struct PageScene {
    lines: Vec<String>,
    text_engine: TextEngine,
}

impl PageScene {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            text_engine: TextEngine::new(),
        }
    }
}

impl RenderScene for PageScene {
    fn render(&mut self, canvas: &mut Canvas) {
        clear(canvas, Rgba::WHITE);
        let bottom = canvas.height() as f32;
        let mut top = MARGIN;
        for line in &self.lines {
            if top > bottom {
                break;
            }
            if line.is_empty() {
                top += STYLE.line_height;
                continue;
            }
            let origin = Point::new(MARGIN, top);
            top += draw_text(&mut self.text_engine, canvas, origin, line, STYLE);
        }
    }
}
