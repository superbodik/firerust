use fr_geom::color::rgba::Rgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Backdrop {
    pub color: Rgba,
    pub left: f32,
}
