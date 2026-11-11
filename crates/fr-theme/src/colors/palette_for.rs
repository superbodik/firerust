use fr_geom::color::rgba::Rgba;

use crate::colors::palette::Palette;
use crate::kind::theme_kind::ThemeKind;

const fn rgb(r: u8, g: u8, b: u8) -> Rgba {
    Rgba::opaque(r, g, b)
}

const LIGHT: Palette = Palette {
    strip_background: rgb(232, 227, 219),
    tab_active: rgb(251, 249, 245),
    tab_hover: rgb(222, 216, 207),
    tab_text: rgb(38, 33, 28),
    tab_text_muted: rgb(126, 117, 106),
    toolbar_background: rgb(251, 249, 245),
    divider: rgb(214, 207, 197),
    field_background: rgb(241, 237, 230),
    field_hover: rgb(235, 230, 222),
    field_focused: rgb(255, 254, 251),
    field_border: rgb(221, 214, 204),
    field_text: rgb(38, 33, 28),
    field_hint: rgb(126, 117, 106),
    selection: rgb(247, 205, 178),
    icon: rgb(78, 69, 60),
    icon_disabled: rgb(184, 175, 163),
    button_hover: Rgba::new(60, 40, 20, 24),
    accent: rgb(205, 72, 22),
    close_hover: rgb(232, 17, 35),
    close_hover_icon: rgb(255, 255, 255),
    star_on: rgb(224, 150, 20),
    secure: rgb(46, 125, 78),
    warning: rgb(190, 96, 20),
    loading_track: rgb(221, 214, 204),
    notice_background: rgb(44, 38, 33),
    notice_text: rgb(246, 241, 232),
    scrollbar: Rgba::new(60, 45, 30, 110),
    scrollbar_hover: Rgba::new(60, 45, 30, 170),
    page_background: rgb(251, 249, 245),
    page_text: rgb(38, 33, 28),
    page_muted: rgb(116, 107, 96),
    page_card: rgb(255, 254, 251),
    page_border: rgb(224, 217, 207),
    page_link: rgb(186, 62, 16),
    key_background: rgb(243, 239, 232),
    key_border: rgb(210, 202, 191),
};

const DARK: Palette = Palette {
    strip_background: rgb(24, 22, 20),
    tab_active: rgb(40, 37, 34),
    tab_hover: rgb(32, 30, 27),
    tab_text: rgb(238, 232, 223),
    tab_text_muted: rgb(152, 144, 133),
    toolbar_background: rgb(40, 37, 34),
    divider: rgb(20, 18, 16),
    field_background: rgb(30, 28, 25),
    field_hover: rgb(26, 24, 22),
    field_focused: rgb(21, 19, 17),
    field_border: rgb(56, 52, 47),
    field_text: rgb(238, 232, 223),
    field_hint: rgb(152, 144, 133),
    selection: rgb(120, 62, 34),
    icon: rgb(214, 207, 197),
    icon_disabled: rgb(96, 90, 82),
    button_hover: Rgba::new(255, 240, 220, 26),
    accent: rgb(255, 125, 64),
    close_hover: rgb(232, 17, 35),
    close_hover_icon: rgb(255, 255, 255),
    star_on: rgb(255, 196, 70),
    secure: rgb(104, 200, 140),
    warning: rgb(255, 170, 80),
    loading_track: rgb(30, 28, 25),
    notice_background: rgb(238, 232, 223),
    notice_text: rgb(34, 30, 26),
    scrollbar: Rgba::new(235, 225, 210, 100),
    scrollbar_hover: Rgba::new(235, 225, 210, 160),
    page_background: rgb(30, 28, 25),
    page_text: rgb(234, 228, 219),
    page_muted: rgb(154, 146, 135),
    page_card: rgb(40, 37, 34),
    page_border: rgb(62, 58, 52),
    page_link: rgb(255, 146, 96),
    key_background: rgb(46, 43, 39),
    key_border: rgb(72, 67, 60),
};

pub const fn palette_for(kind: ThemeKind) -> &'static Palette {
    match kind {
        ThemeKind::Light => &LIGHT,
        ThemeKind::Dark => &DARK,
    }
}
