use fr_geom::color::rgba::Rgba;

pub fn mix_colors(from: Rgba, toward: Rgba, share: f32) -> Rgba {
    let blend = |first: u8, second: u8| {
        (f32::from(first) * (1.0 - share) + f32::from(second) * share).round() as u8
    };
    Rgba::opaque(
        blend(from.r, toward.r),
        blend(from.g, toward.g),
        blend(from.b, toward.b),
    )
}
