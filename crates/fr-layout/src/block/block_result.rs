use crate::boxes::layout_box::LayoutBox;

#[derive(Debug)]
pub struct BlockResult {
    pub layout_box: LayoutBox,
    pub margin_bottom: f32,
}
