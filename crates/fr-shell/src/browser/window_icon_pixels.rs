use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use fr_icons::catalog::icon::Icon;
use fr_raster::draw::clear::clear;
use fr_raster::draw::fill_rounded_rect::fill_rounded_rect;
use fr_raster::surface::canvas::Canvas;
use fr_window::app::window_icon::WindowIcon;

use crate::toolbar::draw_icon::draw_icon;

const SIZE: u32 = 64;
const BACKGROUND: Rgba = Rgba::opaque(205, 72, 22);
const FLAME: Rgba = Rgba::opaque(252, 244, 232);
const CORNER_SHARE: f32 = 0.24;
const FLAME_SHARE: f32 = 0.66;

pub fn window_icon_pixels() -> Option<WindowIcon> {
    let mut canvas = Canvas::new(SIZE, SIZE)?;
    clear(&mut canvas, Rgba::new(0, 0, 0, 0));
    let side = SIZE as f32;
    let area = Rect::from_xywh(0.0, 0.0, side, side);
    fill_rounded_rect(&mut canvas, area, side * CORNER_SHARE, BACKGROUND);
    draw_icon(&mut canvas, Icon::Fire, area, side * FLAME_SHARE, FLAME);
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for pixel in canvas.pixmap().pixels() {
        let straight = pixel.demultiply();
        rgba.extend_from_slice(&[
            straight.red(),
            straight.green(),
            straight.blue(),
            straight.alpha(),
        ]);
    }
    Some(WindowIcon {
        rgba,
        width: SIZE,
        height: SIZE,
    })
}
