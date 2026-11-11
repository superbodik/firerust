use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_icons::catalog::icon::Icon;
use fr_icons::shape::icon_shape::ICON_VIEW_SIZE;
use fr_icons::shape::icon_shape_of::icon_shape_of;
use fr_raster::draw::fill_vector_path::fill_vector_path;
use fr_raster::surface::canvas::Canvas;

pub fn draw_icon(canvas: &mut Canvas, icon: Icon, area: Rect, size: f32, color: Rgba) {
    let scale = size / ICON_VIEW_SIZE;
    let origin = Point::new(
        area.origin.x + (area.size.width - size) / 2.0,
        area.origin.y + (area.size.height - size) / 2.0,
    );
    for part in &icon_shape_of(icon).paths {
        fill_vector_path(canvas, &part.path, origin, scale, color, part.even_odd);
    }
}
