use crate::boxes::layout_box::LayoutBox;
use crate::flow::float_band::FloatBand;
use crate::inline::inline_buffer::InlineBuffer;
use crate::metrics::content_area::ContentArea;

#[derive(Debug)]
pub struct FlowState {
    pub area: ContentArea,
    pub cursor: f32,
    pub pending_margin: f32,
    pub boxes: Vec<LayoutBox>,
    pub inline: InlineBuffer,
    pub has_placed: bool,
    pub absorbs_top: bool,
    pub floats: Vec<FloatBand>,
}

impl FlowState {
    pub fn new(area: ContentArea, absorbs_top: bool) -> Self {
        Self {
            area,
            cursor: area.y,
            pending_margin: 0.0,
            boxes: Vec::new(),
            inline: InlineBuffer::new(),
            has_placed: false,
            absorbs_top,
            floats: Vec::new(),
        }
    }
}
