use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::length::Length;
use fr_geom::geometry::corner_radii::CornerRadii;

pub fn resolve_radii(style: &ComputedStyle, width: f32, height: f32) -> CornerRadii {
    let base = width.min(height);
    let resolve = |length: Length| match length {
        Length::Px(value) => value.max(0.0),
        Length::Percent(percent) => (base * percent / 100.0).max(0.0),
        Length::Em(_) | Length::Rem(_) | Length::Auto => 0.0,
    };
    let radii = CornerRadii {
        top_left: resolve(style.border_radius.top_left),
        top_right: resolve(style.border_radius.top_right),
        bottom_right: resolve(style.border_radius.bottom_right),
        bottom_left: resolve(style.border_radius.bottom_left),
    };
    radii.fitted(width, height)
}
