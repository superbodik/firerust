use fr_geom::geometry::point::Point;
use fr_geom::vector::path_command::PathCommand;
use fr_geom::vector::vector_path::VectorPath;

use crate::parse::arc_to_cubics::{ArcSpec, arc_to_cubics};
use crate::parse::path_reader::PathReader;

struct Pen {
    current: Point,
    start: Point,
    last_cubic: Option<Point>,
    last_quad: Option<Point>,
}

fn reflect(around: Point, control: Option<Point>) -> Point {
    match control {
        Some(point) => Point::new(2.0 * around.x - point.x, 2.0 * around.y - point.y),
        None => around,
    }
}

fn read_point(reader: &mut PathReader, relative: bool, pen: &Pen) -> Option<Point> {
    let x = reader.number()?;
    let y = reader.number()?;
    Some(if relative {
        Point::new(pen.current.x + x, pen.current.y + y)
    } else {
        Point::new(x, y)
    })
}

pub fn parse_svg_path(data: &str) -> Option<VectorPath> {
    let mut reader = PathReader::new(data);
    let mut path = VectorPath::new();
    let mut pen = Pen {
        current: Point::new(0.0, 0.0),
        start: Point::new(0.0, 0.0),
        last_cubic: None,
        last_quad: None,
    };
    let mut command = reader.next_command()?;
    loop {
        let relative = command.is_ascii_lowercase();
        let mut cubic = None;
        let mut quad = None;
        match command.to_ascii_uppercase() {
            'M' => {
                let target = read_point(&mut reader, relative, &pen)?;
                path.push(PathCommand::MoveTo(target));
                pen.current = target;
                pen.start = target;
                command = if relative { 'l' } else { 'L' };
            }
            'L' => {
                let target = read_point(&mut reader, relative, &pen)?;
                path.push(PathCommand::LineTo(target));
                pen.current = target;
            }
            'H' => {
                let value = reader.number()?;
                let x = if relative {
                    pen.current.x + value
                } else {
                    value
                };
                pen.current = Point::new(x, pen.current.y);
                path.push(PathCommand::LineTo(pen.current));
            }
            'V' => {
                let value = reader.number()?;
                let y = if relative {
                    pen.current.y + value
                } else {
                    value
                };
                pen.current = Point::new(pen.current.x, y);
                path.push(PathCommand::LineTo(pen.current));
            }
            'C' => {
                let first = read_point(&mut reader, relative, &pen)?;
                let second = read_point(&mut reader, relative, &pen)?;
                let target = read_point(&mut reader, relative, &pen)?;
                path.push(PathCommand::CubicTo(first, second, target));
                pen.current = target;
                cubic = Some(second);
            }
            'S' => {
                let first = reflect(pen.current, pen.last_cubic);
                let second = read_point(&mut reader, relative, &pen)?;
                let target = read_point(&mut reader, relative, &pen)?;
                path.push(PathCommand::CubicTo(first, second, target));
                pen.current = target;
                cubic = Some(second);
            }
            'Q' => {
                let control = read_point(&mut reader, relative, &pen)?;
                let target = read_point(&mut reader, relative, &pen)?;
                path.push(PathCommand::QuadTo(control, target));
                pen.current = target;
                quad = Some(control);
            }
            'T' => {
                let control = reflect(pen.current, pen.last_quad);
                let target = read_point(&mut reader, relative, &pen)?;
                path.push(PathCommand::QuadTo(control, target));
                pen.current = target;
                quad = Some(control);
            }
            'A' => {
                let radius_x = reader.number()?;
                let radius_y = reader.number()?;
                let rotation = reader.number()?;
                let large_arc = reader.flag()?;
                let sweep = reader.flag()?;
                let target = read_point(&mut reader, relative, &pen)?;
                let pieces = arc_to_cubics(&ArcSpec {
                    from: pen.current,
                    to: target,
                    radius_x,
                    radius_y,
                    rotation_degrees: rotation,
                    large_arc,
                    sweep,
                });
                if pieces.is_empty() {
                    path.push(PathCommand::LineTo(target));
                }
                for (first, second, end) in pieces {
                    path.push(PathCommand::CubicTo(first, second, end));
                }
                pen.current = target;
            }
            'Z' => {
                path.push(PathCommand::Close);
                pen.current = pen.start;
            }
            _ => return None,
        }
        pen.last_cubic = cubic;
        pen.last_quad = quad;
        if reader.is_finished() {
            break;
        }
        if let Some(next) = reader.next_command() {
            command = next;
        } else if !reader.has_number() || command.eq_ignore_ascii_case(&'z') {
            return None;
        }
    }
    Some(path)
}
