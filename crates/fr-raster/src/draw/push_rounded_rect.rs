use fr_geom::geometry::corner_radii::CornerRadii;
use fr_geom::geometry::rect::Rect;
use tiny_skia::PathBuilder;

const KAPPA: f32 = 0.552_284_8;

pub fn push_rounded_rect(builder: &mut PathBuilder, rect: Rect, radii: CornerRadii) {
    let radii = radii.fitted(rect.size.width, rect.size.height);
    let left = rect.origin.x;
    let top = rect.origin.y;
    let right = rect.right();
    let bottom = rect.bottom();
    let tl = radii.top_left;
    let tr = radii.top_right;
    let br = radii.bottom_right;
    let bl = radii.bottom_left;

    builder.move_to(left + tl, top);
    builder.line_to(right - tr, top);
    builder.cubic_to(
        right - tr + tr * KAPPA,
        top,
        right,
        top + tr - tr * KAPPA,
        right,
        top + tr,
    );
    builder.line_to(right, bottom - br);
    builder.cubic_to(
        right,
        bottom - br + br * KAPPA,
        right - br + br * KAPPA,
        bottom,
        right - br,
        bottom,
    );
    builder.line_to(left + bl, bottom);
    builder.cubic_to(
        left + bl - bl * KAPPA,
        bottom,
        left,
        bottom - bl + bl * KAPPA,
        left,
        bottom - bl,
    );
    builder.line_to(left, top + tl);
    builder.cubic_to(
        left,
        top + tl - tl * KAPPA,
        left + tl - tl * KAPPA,
        top,
        left + tl,
        top,
    );
    builder.close();
}
