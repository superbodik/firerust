use crate::boxes::layout_box::LayoutBox;

#[derive(Debug)]
pub struct FlowResult {
    pub boxes: Vec<LayoutBox>,
    pub content_height: f32,
    pub margin_bottom_out: f32,
}
