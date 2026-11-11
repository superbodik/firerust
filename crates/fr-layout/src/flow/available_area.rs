use crate::flow::flow_state::FlowState;
use crate::flow::narrowed_area::narrowed_area;
use crate::flow::nearest_float_bottom::nearest_float_bottom;
use crate::metrics::content_area::ContentArea;

const COMFORTABLE_WIDTH: f32 = 140.0;
const COMFORTABLE_SHARE: f32 = 0.35;
const LEAST_WIDTH: f32 = 24.0;

pub fn available_area(state: &mut FlowState) -> ContentArea {
    let needed = (state.area.width * COMFORTABLE_SHARE).clamp(LEAST_WIDTH, COMFORTABLE_WIDTH);
    loop {
        let area = narrowed_area(state, state.cursor);
        if area.width >= needed {
            return area;
        }
        let Some(next) = nearest_float_bottom(state, state.cursor) else {
            return area;
        };
        state.cursor = next;
    }
}
