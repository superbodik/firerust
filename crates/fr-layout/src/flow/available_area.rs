use crate::flow::flow_state::FlowState;
use crate::flow::narrowed_area::narrowed_area;
use crate::flow::nearest_float_bottom::nearest_float_bottom;
use crate::metrics::content_area::ContentArea;

const MIN_BESIDE_FLOATS: f32 = 140.0;
const MIN_CONTAINER_TO_CLEAR: f32 = 280.0;

pub fn available_area(state: &mut FlowState) -> ContentArea {
    loop {
        let area = narrowed_area(state, state.cursor);
        if area.width >= MIN_BESIDE_FLOATS || state.area.width < MIN_CONTAINER_TO_CLEAR {
            return area;
        }
        let Some(next) = nearest_float_bottom(state, state.cursor) else {
            return area;
        };
        state.cursor = next;
    }
}
