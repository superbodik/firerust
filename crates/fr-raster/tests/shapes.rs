use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_geom::vector::path_command::PathCommand;
use fr_geom::vector::vector_path::VectorPath;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::draw::fill_vector_path::fill_vector_path;
use fr_raster::draw::stroke_rounded_rect::stroke_rounded_rect;
use fr_raster::surface::canvas::Canvas;

const WHITE: Rgba = Rgba::opaque(255, 255, 255);
const RED: Rgba = Rgba::opaque(255, 0, 0);

fn canvas() -> Canvas {
    let mut canvas = Canvas::new(40, 40).expect("canvas");
    clear(&mut canvas, WHITE);
    canvas
}

fn rgb(canvas: &Canvas, x: u32, y: u32) -> (u8, u8, u8) {
    let pixel = canvas.pixmap().pixel(x, y).expect("pixel");
    (pixel.red(), pixel.green(), pixel.blue())
}

fn square(size: f32) -> VectorPath {
    let mut path = VectorPath::new();
    path.push(PathCommand::MoveTo(Point::new(0.0, 0.0)));
    path.push(PathCommand::LineTo(Point::new(size, 0.0)));
    path.push(PathCommand::LineTo(Point::new(size, size)));
    path.push(PathCommand::LineTo(Point::new(0.0, size)));
    path.push(PathCommand::Close);
    path
}

#[test]
fn a_rounded_rect_is_filled_inside_and_cut_at_the_corners() {
    let mut canvas = canvas();

    fill_rounded_rect(
        &mut canvas,
        Rect::from_xywh(4.0, 4.0, 32.0, 32.0),
        10.0,
        RED,
    );

    assert_eq!(rgb(&canvas, 20, 20), (255, 0, 0));
    assert_eq!(rgb(&canvas, 4, 4), (255, 255, 255));
    assert_eq!(rgb(&canvas, 20, 5), (255, 0, 0));
    assert_eq!(rgb(&canvas, 2, 20), (255, 255, 255));
}

#[test]
fn a_huge_radius_makes_a_circle() {
    let mut canvas = canvas();

    fill_rounded_rect(
        &mut canvas,
        Rect::from_xywh(4.0, 4.0, 32.0, 32.0),
        500.0,
        RED,
    );

    assert_eq!(rgb(&canvas, 20, 20), (255, 0, 0));
    assert_eq!(rgb(&canvas, 6, 6), (255, 255, 255));
}

#[test]
fn a_stroked_rect_leaves_the_middle_alone() {
    let mut canvas = canvas();

    stroke_rounded_rect(
        &mut canvas,
        Rect::from_xywh(4.0, 4.0, 32.0, 32.0),
        6.0,
        2.0,
        RED,
    );

    assert_eq!(rgb(&canvas, 20, 20), (255, 255, 255));
    let edge = rgb(&canvas, 20, 4);
    assert!(
        edge.1 < 200,
        "the top edge should be coloured, got {edge:?}"
    );
}

#[test]
fn a_vector_path_is_scaled_and_moved() {
    let mut canvas = canvas();

    fill_vector_path(
        &mut canvas,
        &square(4.0),
        Point::new(10.0, 10.0),
        2.0,
        RED,
        false,
    );

    assert_eq!(rgb(&canvas, 14, 14), (255, 0, 0));
    assert_eq!(rgb(&canvas, 9, 14), (255, 255, 255));
    assert_eq!(rgb(&canvas, 19, 14), (255, 255, 255));
}

#[test]
fn the_even_odd_rule_cuts_a_hole() {
    let mut path = square(16.0);
    path.push(PathCommand::MoveTo(Point::new(4.0, 4.0)));
    path.push(PathCommand::LineTo(Point::new(12.0, 4.0)));
    path.push(PathCommand::LineTo(Point::new(12.0, 12.0)));
    path.push(PathCommand::LineTo(Point::new(4.0, 12.0)));
    path.push(PathCommand::Close);
    let mut canvas = canvas();

    fill_vector_path(&mut canvas, &path, Point::new(0.0, 0.0), 1.0, RED, true);

    assert_eq!(rgb(&canvas, 2, 2), (255, 0, 0));
    assert_eq!(rgb(&canvas, 8, 8), (255, 255, 255));
}

#[test]
fn blitting_places_a_smaller_canvas_at_an_offset() {
    use fr_raster::draw::blit_canvas::blit_canvas;

    let mut target = canvas();
    let mut source = Canvas::new(10, 10).expect("canvas");
    clear(&mut source, RED);

    blit_canvas(&mut target, &source, 5, 8);

    assert_eq!(rgb(&target, 6, 9), (255, 0, 0));
    assert_eq!(rgb(&target, 14, 17), (255, 0, 0));
    assert_eq!(rgb(&target, 4, 9), (255, 255, 255));
    assert_eq!(rgb(&target, 6, 7), (255, 255, 255));
    assert_eq!(rgb(&target, 15, 18), (255, 255, 255));
}
