use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_page::lines::line::Line;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rect::fill_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_run::TextRun;
use fr_window::scene::render_scene::RenderScene;

pub struct PageScene {
    lines: Vec<Line>,
    background: Rgba,
    text_engine: TextEngine,
}

impl PageScene {
    pub fn new(lines: Vec<Line>, background: Rgba) -> Self {
        Self {
            lines,
            background,
            text_engine: TextEngine::new(),
        }
    }
}

impl RenderScene for PageScene {
    fn render(&mut self, canvas: &mut Canvas) {
        clear(canvas, self.background);
        let bottom = canvas.height() as f32;
        let canvas_width = canvas.width() as f32;
        let mut top = 0.0;
        for line in &self.lines {
            top += line.gap_before;
            if top > bottom {
                break;
            }
            let left = line.indent;
            let width = (canvas_width - left).max(1.0);
            let runs: Vec<TextRun<'_>> = line
                .runs
                .iter()
                .map(|run| TextRun {
                    text: &run.text,
                    color: run.style.color,
                    font_size: run.style.font_size,
                    font_weight: run.style.font_weight,
                })
                .collect();
            let mut shaped = shape_runs(&mut self.text_engine, &runs, width);
            let height = shaped.height();
            if let Some(backdrop) = line.background {
                let rect = Rect::from_xywh(
                    backdrop.left,
                    top,
                    (canvas_width - backdrop.left).max(1.0),
                    height,
                );
                fill_rect(canvas, rect, backdrop.color);
            }
            shaped.draw(&mut self.text_engine, canvas, Point::new(left, top));
            top += height;
        }
    }
}
