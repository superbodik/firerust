use fr_css::value::float_side::FloatSide;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatBand {
    pub side: FloatSide,
    pub width: f32,
    pub bottom: f32,
}
