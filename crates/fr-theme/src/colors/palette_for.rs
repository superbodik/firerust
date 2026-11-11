use fr_geom::color::rgba::Rgba;

use crate::colors::palette::Palette;
use crate::kind::theme_kind::ThemeKind;

const fn rgb(r: u8, g: u8, b: u8) -> Rgba {
    Rgba::opaque(r, g, b)
}

const LIGHT: Palette = Palette {
    strip_background: rgb(228, 231, 238),
    tab_active: rgb(250, 251, 253),
    tab_hover: rgb(214, 218, 228),
    tab_text: rgb(28, 30, 38),
    tab_text_muted: rgb(112, 117, 132),
    toolbar_background: rgb(250, 251, 253),
    divider: rgb(214, 218, 228),
    field_background: rgb(236, 239, 245),
    field_hover: rgb(228, 232, 240),
    field_focused: rgb(255, 255, 255),
    field_text: rgb(28, 30, 38),
    field_hint: rgb(112, 117, 132),
    selection: rgb(176, 208, 255),
    icon: rgb(58, 63, 75),
    icon_disabled: rgb(178, 183, 196),
    button_hover: Rgba::new(20, 30, 60, 22),
    accent: rgb(52, 120, 246),
    close_hover: rgb(232, 17, 35),
    close_hover_icon: rgb(255, 255, 255),
    star_on: rgb(240, 160, 20),
    secure: rgb(34, 139, 84),
    warning: rgb(200, 110, 20),
    loading_track: rgb(214, 218, 228),
    notice_background: rgb(38, 41, 52),
    notice_text: rgb(240, 242, 248),
    scrollbar: Rgba::new(40, 46, 66, 110),
    scrollbar_hover: Rgba::new(40, 46, 66, 170),
    page_background: rgb(246, 244, 239),
    page_text: rgb(34, 36, 44),
    page_muted: rgb(102, 106, 120),
    page_card: rgb(255, 255, 255),
    page_border: rgb(224, 222, 214),
    page_link: rgb(11, 92, 173),
};

const DARK: Palette = Palette {
    strip_background: rgb(24, 26, 32),
    tab_active: rgb(42, 45, 56),
    tab_hover: rgb(36, 39, 48),
    tab_text: rgb(232, 234, 241),
    tab_text_muted: rgb(140, 146, 162),
    toolbar_background: rgb(42, 45, 56),
    divider: rgb(18, 20, 25),
    field_background: rgb(30, 32, 40),
    field_hover: rgb(24, 26, 33),
    field_focused: rgb(20, 22, 28),
    field_text: rgb(232, 234, 241),
    field_hint: rgb(140, 146, 162),
    selection: rgb(52, 92, 160),
    icon: rgb(208, 212, 224),
    icon_disabled: rgb(92, 97, 112),
    button_hover: Rgba::new(255, 255, 255, 26),
    accent: rgb(110, 162, 255),
    close_hover: rgb(232, 17, 35),
    close_hover_icon: rgb(255, 255, 255),
    star_on: rgb(255, 196, 70),
    secure: rgb(96, 200, 140),
    warning: rgb(255, 170, 80),
    loading_track: rgb(30, 32, 40),
    notice_background: rgb(232, 234, 241),
    notice_text: rgb(28, 30, 38),
    scrollbar: Rgba::new(220, 226, 245, 100),
    scrollbar_hover: Rgba::new(220, 226, 245, 160),
    page_background: rgb(26, 28, 35),
    page_text: rgb(228, 230, 238),
    page_muted: rgb(150, 156, 172),
    page_card: rgb(38, 41, 52),
    page_border: rgb(56, 60, 74),
    page_link: rgb(124, 176, 255),
};

pub const fn palette_for(kind: ThemeKind) -> &'static Palette {
    match kind {
        ThemeKind::Light => &LIGHT,
        ThemeKind::Dark => &DARK,
    }
}
