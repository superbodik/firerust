use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use fr_icons::catalog::icon::Icon;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_theme::colors::palette::Palette;

use crate::toolbar::draw_icon::draw_icon;

const CORNER: f32 = 6.0;

pub fn draw_icon_button(
    canvas: &mut Canvas,
    palette: &Palette,
    area: Rect,
    icon: Icon,
    icon_size: f32,
    color: Rgba,
    hovered: bool,
) {
    if hovered {
        fill_rounded_rect(canvas, area, CORNER, palette.button_hover);
    }
    draw_icon(canvas, icon, area, icon_size, color);
}
