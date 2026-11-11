use fr_theme::colors::palette_for::palette_for;
use fr_theme::kind::theme_kind::ThemeKind;
use fr_theme::page::internal_css::internal_css;

fn luminance(r: u8, g: u8, b: u8) -> f32 {
    0.2126 * f32::from(r) + 0.7152 * f32::from(g) + 0.0722 * f32::from(b)
}

fn contrast(first: (u8, u8, u8), second: (u8, u8, u8)) -> f32 {
    (luminance(first.0, first.1, first.2) - luminance(second.0, second.1, second.2)).abs()
}

#[test]
fn the_themes_are_different_and_the_kind_converts() {
    assert_ne!(palette_for(ThemeKind::Light), palette_for(ThemeKind::Dark));
    assert!(ThemeKind::from_dark(true).is_dark());
    assert!(!ThemeKind::from_dark(false).is_dark());
    assert_eq!(ThemeKind::default(), ThemeKind::Light);
}

#[test]
fn text_stays_readable_on_its_backgrounds_in_both_themes() {
    for kind in [ThemeKind::Light, ThemeKind::Dark] {
        let palette = palette_for(kind);
        let pairs = [
            (palette.tab_text, palette.tab_active),
            (palette.tab_text, palette.strip_background),
            (palette.field_text, palette.field_background),
            (palette.field_text, palette.field_focused),
            (palette.icon, palette.toolbar_background),
            (palette.page_text, palette.page_background),
            (palette.page_text, palette.page_card),
            (palette.notice_text, palette.notice_background),
        ];
        for (text, background) in pairs {
            let distance = contrast(
                (text.r, text.g, text.b),
                (background.r, background.g, background.b),
            );
            assert!(distance > 90.0, "{kind:?}: contrast {distance}");
        }
    }
}

#[test]
fn the_page_css_uses_the_palette_colours() {
    let light = internal_css(palette_for(ThemeKind::Light));
    let dark = internal_css(palette_for(ThemeKind::Dark));

    assert!(light.contains("#f6f4ef"));
    assert!(dark.contains("#1a1c23"));
    assert_ne!(light, dark);
}
