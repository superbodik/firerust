use fr_css::parse::declaration::Declaration;
use fr_css::parse::parse_inline_style::parse_inline_style;
use fr_geom::color::rgba::Rgba;

fn color_of(css: &str) -> Option<Rgba> {
    parse_inline_style(css)
        .into_iter()
        .find_map(|entry| match entry.declaration {
            Declaration::Color(color) => Some(color),
            _ => None,
        })
}

#[test]
fn a_short_hex_color_is_expanded() {
    assert_eq!(color_of("color: #f80"), Some(Rgba::opaque(255, 136, 0)));
}

#[test]
fn a_long_hex_color_is_read() {
    assert_eq!(color_of("color: #0b5394"), Some(Rgba::opaque(11, 83, 148)));
}

#[test]
fn a_hex_color_with_alpha_is_read() {
    assert_eq!(
        color_of("color: #ff880080"),
        Some(Rgba::new(255, 136, 0, 128))
    );
}

#[test]
fn a_named_color_is_case_insensitive() {
    assert_eq!(color_of("color: Red"), Some(Rgba::opaque(255, 0, 0)));
}

#[test]
fn transparent_has_no_alpha() {
    assert_eq!(color_of("color: transparent"), Some(Rgba::new(0, 0, 0, 0)));
}

#[test]
fn rgb_with_commas_is_read() {
    assert_eq!(
        color_of("color: rgb(10, 20, 30)"),
        Some(Rgba::opaque(10, 20, 30))
    );
}

#[test]
fn rgb_with_spaces_and_alpha_is_read() {
    assert_eq!(
        color_of("color: rgb(10 20 30 / 50%)"),
        Some(Rgba::new(10, 20, 30, 128))
    );
}

#[test]
fn rgba_with_a_number_alpha_is_read() {
    assert_eq!(
        color_of("color: rgba(255, 0, 0, 0.5)"),
        Some(Rgba::new(255, 0, 0, 128))
    );
}

#[test]
fn rgb_channels_are_clamped() {
    assert_eq!(
        color_of("color: rgb(300, -5, 10)"),
        Some(Rgba::opaque(255, 0, 10))
    );
}

#[test]
fn an_unknown_color_is_dropped() {
    assert_eq!(color_of("color: banana"), None);
    assert_eq!(color_of("color: #12"), None);
    assert_eq!(color_of("color: rgb(1, 2)"), None);
}
