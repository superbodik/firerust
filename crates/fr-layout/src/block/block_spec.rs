#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockSpec {
    pub x: f32,
    pub y: f32,
    pub available_width: f32,
    pub forced_width: Option<f32>,
    pub forced_height: Option<f32>,
}
