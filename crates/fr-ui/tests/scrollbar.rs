use fr_ui::scroll::scroll_metrics::ScrollMetrics;
use fr_ui::scroll::scrollbar_thumb::scrollbar_thumb;

#[test]
fn a_short_page_has_no_thumb() {
    assert!(scrollbar_thumb(0.0, ScrollMetrics::new(300.0, 600.0), 800.0).is_none());
}

#[test]
fn the_thumb_height_is_the_visible_share_of_the_page() {
    let thumb = scrollbar_thumb(0.0, ScrollMetrics::new(1200.0, 600.0), 800.0).unwrap();

    assert_eq!(thumb.size.height, 300.0);
}

#[test]
fn the_thumb_sits_at_the_top_when_not_scrolled() {
    let thumb = scrollbar_thumb(0.0, ScrollMetrics::new(1200.0, 600.0), 800.0).unwrap();

    assert_eq!(thumb.origin.y, 0.0);
}

#[test]
fn the_thumb_reaches_the_bottom_at_the_maximum_offset() {
    let thumb = scrollbar_thumb(600.0, ScrollMetrics::new(1200.0, 600.0), 800.0).unwrap();

    assert_eq!(thumb.bottom(), 600.0);
}

#[test]
fn the_thumb_hugs_the_right_edge() {
    let thumb = scrollbar_thumb(0.0, ScrollMetrics::new(1200.0, 600.0), 800.0).unwrap();

    assert_eq!(thumb.right(), 798.0);
}

#[test]
fn a_very_long_page_keeps_a_grabbable_thumb() {
    let thumb = scrollbar_thumb(0.0, ScrollMetrics::new(1_000_000.0, 600.0), 800.0).unwrap();

    assert_eq!(thumb.size.height, 24.0);
}
