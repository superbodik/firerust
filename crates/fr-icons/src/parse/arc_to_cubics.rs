use std::f64::consts::{FRAC_PI_2, TAU};

use fr_geom::geometry::point::Point;

pub struct ArcSpec {
    pub from: Point,
    pub to: Point,
    pub radius_x: f32,
    pub radius_y: f32,
    pub rotation_degrees: f32,
    pub large_arc: bool,
    pub sweep: bool,
}

pub fn arc_to_cubics(arc: &ArcSpec) -> Vec<(Point, Point, Point)> {
    let (x1, y1) = (f64::from(arc.from.x), f64::from(arc.from.y));
    let (x2, y2) = (f64::from(arc.to.x), f64::from(arc.to.y));
    let mut rx = f64::from(arc.radius_x).abs();
    let mut ry = f64::from(arc.radius_y).abs();
    if (x1 == x2 && y1 == y2) || rx == 0.0 || ry == 0.0 {
        return Vec::new();
    }
    let phi = f64::from(arc.rotation_degrees).to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let half_dx = (x1 - x2) / 2.0;
    let half_dy = (y1 - y2) / 2.0;
    let x1p = cos_phi * half_dx + sin_phi * half_dy;
    let y1p = -sin_phi * half_dx + cos_phi * half_dy;
    let scale = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if scale > 1.0 {
        let factor = scale.sqrt();
        rx *= factor;
        ry *= factor;
    }
    let numerator = rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p;
    let denominator = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    let sign = if arc.large_arc == arc.sweep {
        -1.0
    } else {
        1.0
    };
    let coefficient = sign * (numerator / denominator).max(0.0).sqrt();
    let cxp = coefficient * rx * y1p / ry;
    let cyp = -coefficient * ry * x1p / rx;
    let center_x = cos_phi * cxp - sin_phi * cyp + (x1 + x2) / 2.0;
    let center_y = sin_phi * cxp + cos_phi * cyp + (y1 + y2) / 2.0;
    let start = ((x1p - cxp) / rx, (y1p - cyp) / ry);
    let end = ((-x1p - cxp) / rx, (-y1p - cyp) / ry);
    let theta = angle_between((1.0, 0.0), start);
    let mut sweep_angle = angle_between(start, end);
    if !arc.sweep && sweep_angle > 0.0 {
        sweep_angle -= TAU;
    } else if arc.sweep && sweep_angle < 0.0 {
        sweep_angle += TAU;
    }
    let pieces = (sweep_angle.abs() / FRAC_PI_2).ceil().max(1.0) as usize;
    let delta = sweep_angle / pieces as f64;
    let handle = 4.0 / 3.0 * (delta / 4.0).tan();
    let map = |x: f64, y: f64| {
        Point::new(
            (center_x + rx * cos_phi * x - ry * sin_phi * y) as f32,
            (center_y + rx * sin_phi * x + ry * cos_phi * y) as f32,
        )
    };
    let mut pieces_out = Vec::with_capacity(pieces);
    for index in 0..pieces {
        let a = theta + delta * index as f64;
        let b = a + delta;
        let (sin_a, cos_a) = a.sin_cos();
        let (sin_b, cos_b) = b.sin_cos();
        pieces_out.push((
            map(cos_a - handle * sin_a, sin_a + handle * cos_a),
            map(cos_b + handle * sin_b, sin_b - handle * cos_b),
            map(cos_b, sin_b),
        ));
    }
    pieces_out
}

fn angle_between(first: (f64, f64), second: (f64, f64)) -> f64 {
    let cross = first.0 * second.1 - first.1 * second.0;
    let dot = first.0 * second.0 + first.1 * second.1;
    cross.atan2(dot)
}
