use fr_geom::color::rgba::Rgba;

use crate::boxes::layout_box::LayoutBox;

#[derive(Debug, Clone)]
pub struct LayoutTree {
    pub root: LayoutBox,
    pub height: f32,
    pub background: Rgba,
}
