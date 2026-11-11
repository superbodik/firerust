use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::vector::path_command::PathCommand;
use fr_geom::vector::vector_path::VectorPath;
use tiny_skia::{FillRule, PathBuilder, Transform};

use crate::draw::smooth_paint::smooth_paint;
use crate::surface::canvas::Canvas;

pub fn fill_vector_path(
    canvas: &mut Canvas,
    path: &VectorPath,
    origin: Point,
    scale: f32,
    color: Rgba,
    even_odd: bool,
) {
    let mut builder = PathBuilder::new();
    for command in &path.commands {
        match command {
            PathCommand::MoveTo(point) => builder.move_to(point.x, point.y),
            PathCommand::LineTo(point) => builder.line_to(point.x, point.y),
            PathCommand::QuadTo(control, point) => {
                builder.quad_to(control.x, control.y, point.x, point.y)
            }
            PathCommand::CubicTo(first, second, point) => {
                builder.cubic_to(first.x, first.y, second.x, second.y, point.x, point.y)
            }
            PathCommand::Close => builder.close(),
        }
    }
    let Some(built) = builder.finish() else {
        return;
    };
    let rule = if even_odd {
        FillRule::EvenOdd
    } else {
        FillRule::Winding
    };
    let transform = Transform::from_scale(scale, scale).post_translate(origin.x, origin.y);
    canvas
        .pixmap_mut()
        .fill_path(&built, &smooth_paint(color), rule, transform, None);
}
