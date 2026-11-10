use fr_ui::zoom::zoom_level::ZoomLevel;

#[test]
fn the_default_level_is_one_hundred_percent() {
    let level = ZoomLevel::new();

    assert_eq!(level.percent(), 100);
    assert!(level.is_default());
    assert!((level.factor() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn zooming_in_walks_up_the_steps() {
    let level = ZoomLevel::new().zoom_in();
    assert_eq!(level.percent(), 110);
    assert_eq!(level.zoom_in().percent(), 125);
    assert!(!level.is_default());
}

#[test]
fn zooming_out_walks_down_the_steps() {
    let level = ZoomLevel::new().zoom_out();
    assert_eq!(level.percent(), 90);
    assert_eq!(level.zoom_out().percent(), 80);
}

#[test]
fn the_levels_stop_at_the_limits() {
    let mut level = ZoomLevel::new();
    for _ in 0..40 {
        level = level.zoom_in();
    }
    assert_eq!(level.percent(), 500);
    assert_eq!(level.zoom_in().percent(), 500);

    let mut level = ZoomLevel::new();
    for _ in 0..40 {
        level = level.zoom_out();
    }
    assert_eq!(level.percent(), 25);
    assert_eq!(level.zoom_out().percent(), 25);
}

#[test]
fn zooming_in_then_out_returns_to_the_start() {
    let level = ZoomLevel::new().zoom_in().zoom_in().zoom_out().zoom_out();

    assert!(level.is_default());
}

#[test]
fn the_factor_follows_the_percent() {
    let level = ZoomLevel::new().zoom_in().zoom_in();

    assert!((level.factor() - 1.25).abs() < 1e-6);
}
