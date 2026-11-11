use fr_geom::geometry::point::Point;
use fr_geom::vector::path_command::PathCommand;
use fr_icons::catalog::icon::Icon;
use fr_icons::parse::extract_paths::extract_paths;
use fr_icons::parse::parse_svg_path::parse_svg_path;
use fr_icons::shape::icon_shape::ICON_VIEW_SIZE;
use fr_icons::shape::icon_shape_of::icon_shape_of;

fn bounds(commands: &[PathCommand]) -> (f32, f32, f32, f32) {
    let mut found = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    let mut add = |point: &Point| {
        found.0 = found.0.min(point.x);
        found.1 = found.1.min(point.y);
        found.2 = found.2.max(point.x);
        found.3 = found.3.max(point.y);
    };
    for command in commands {
        match command {
            PathCommand::MoveTo(a) | PathCommand::LineTo(a) => add(a),
            PathCommand::QuadTo(a, b) => {
                add(a);
                add(b);
            }
            PathCommand::CubicTo(a, b, c) => {
                add(a);
                add(b);
                add(c);
            }
            PathCommand::Close => {}
        }
    }
    found
}

#[test]
fn lines_and_closing_are_read() {
    let path = parse_svg_path("M1 2L3 4H5V6z").expect("path");

    assert_eq!(
        path.commands,
        vec![
            PathCommand::MoveTo(Point::new(1.0, 2.0)),
            PathCommand::LineTo(Point::new(3.0, 4.0)),
            PathCommand::LineTo(Point::new(5.0, 4.0)),
            PathCommand::LineTo(Point::new(5.0, 6.0)),
            PathCommand::Close,
        ]
    );
}

#[test]
fn relative_commands_and_implicit_repeats_work() {
    let path = parse_svg_path("m1 1 2 0 0 2").expect("path");

    assert_eq!(
        path.commands,
        vec![
            PathCommand::MoveTo(Point::new(1.0, 1.0)),
            PathCommand::LineTo(Point::new(3.0, 1.0)),
            PathCommand::LineTo(Point::new(3.0, 3.0)),
        ]
    );
}

#[test]
fn glued_numbers_are_split_correctly() {
    let path = parse_svg_path("M.5.25-1.5-2").expect("path");

    assert_eq!(path.commands[0], PathCommand::MoveTo(Point::new(0.5, 0.25)));
    assert_eq!(
        path.commands[1],
        PathCommand::LineTo(Point::new(-1.5, -2.0))
    );
}

#[test]
fn a_half_circle_arc_stays_on_its_circle() {
    let path = parse_svg_path("M0 0A5 5 0 0 1 10 0").expect("path");

    let (min_x, min_y, max_x, max_y) = bounds(&path.commands);

    assert!((min_x - 0.0).abs() < 0.01 && (max_x - 10.0).abs() < 0.01);
    assert!((min_y - -5.0).abs() < 0.1, "top of the arc was {min_y}");
    assert!(max_y.abs() < 0.01);
}

#[test]
fn a_full_circle_from_two_arcs_is_round() {
    let path = parse_svg_path("M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492")
        .expect("path");

    let (min_x, min_y, max_x, max_y) = bounds(&path.commands);

    assert!((min_x - 4.754).abs() < 0.05 && (max_x - 11.246).abs() < 0.05);
    assert!((min_y - 4.754).abs() < 0.05 && (max_y - 11.246).abs() < 0.05);
}

#[test]
fn broken_data_is_rejected() {
    assert!(parse_svg_path("").is_none());
    assert!(parse_svg_path("M1").is_none());
    assert!(parse_svg_path("X1 2").is_none());
    assert!(parse_svg_path("L1 2").is_some());
}

#[test]
fn the_paths_and_fill_rules_come_out_of_the_svg_text() {
    let svg = "<svg viewBox=\"0 0 16 16\"><path d=\"M1 1h2\"/>\n<path fill-rule=\"evenodd\" d=\"M2 2h3\"/></svg>";

    let found = extract_paths(svg);

    assert_eq!(
        found,
        vec![("M1 1h2".to_owned(), false), ("M2 2h3".to_owned(), true)]
    );
}

#[test]
fn every_icon_has_shapes_inside_the_view_box() {
    for icon in Icon::ALL {
        let shape = icon_shape_of(icon);
        assert!(!shape.paths.is_empty(), "{icon:?} has no paths");
        for part in &shape.paths {
            let (min_x, min_y, max_x, max_y) = bounds(&part.path.commands);
            let limit = ICON_VIEW_SIZE + 1.0;
            assert!(
                min_x >= -1.0 && min_y >= -1.0 && max_x <= limit && max_y <= limit,
                "{icon:?} leaves the view box: {min_x} {min_y} {max_x} {max_y}"
            );
        }
    }
}

#[test]
fn icon_indexes_are_unique() {
    let mut seen = std::collections::HashSet::new();
    for icon in Icon::ALL {
        assert!(seen.insert(icon.index()));
    }
}
