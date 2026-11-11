use crate::flow::flow_state::FlowState;

pub fn nearest_float_bottom(state: &FlowState, after: f32) -> Option<f32> {
    state
        .floats
        .iter()
        .map(|band| band.bottom)
        .filter(|bottom| *bottom > after)
        .min_by(f32::total_cmp)
}
