use fr_icons::catalog::icon::Icon;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::draw::stroke_rounded_rect::stroke_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_theme::colors::palette::Palette;
use fr_ui::menu::start_button_rect::start_button_rect;

use crate::toolbar::draw_icon::draw_icon;
use crate::toolbar::draw_menu_text::draw_menu_text;

const CORNER: f32 = 10.0;
const FONT_SIZE: f32 = 13.0;

pub fn draw_start_button(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    palette: &Palette,
    is_lit: bool,
) {
    let area = start_button_rect(canvas.height() as f32);
    fill_rounded_rect(canvas, area, CORNER, palette.page_card);
    if is_lit {
        fill_rounded_rect(canvas, area, CORNER, palette.button_hover);
    }
    stroke_rounded_rect(canvas, area, CORNER, 1.0, palette.page_border);
    let icon_area = fr_geom::geometry::rect::Rect::from_xywh(
        area.origin.x + 6.0,
        area.origin.y,
        28.0,
        area.size.height,
    );
    draw_icon(canvas, Icon::BookmarkStar, icon_area, 16.0, palette.accent);
    draw_menu_text(
        canvas,
        engine,
        "Shortcuts",
        FONT_SIZE,
        palette.field_text,
        area.origin.x + 38.0,
        area,
    );
}
