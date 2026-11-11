use crate::shape::shape_path::ShapePath;

pub const ICON_VIEW_SIZE: f32 = 16.0;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IconShape {
    pub paths: Vec<ShapePath>,
}
