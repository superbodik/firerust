use fr_css::value::clear_side::ClearSide;
use fr_css::value::float_side::FloatSide;

use crate::flow::flow_state::FlowState;

pub fn apply_clear(state: &mut FlowState, clear: ClearSide) {
    if clear == ClearSide::None {
        return;
    }
    let bottom = state
        .floats
        .iter()
        .filter(|band| match band.side {
            FloatSide::Left => clear.clears_left(),
            FloatSide::Right => clear.clears_right(),
            FloatSide::None => false,
        })
        .map(|band| band.bottom)
        .fold(f32::MIN, f32::max);
    if bottom > state.cursor + state.pending_margin {
        state.cursor = bottom;
        state.pending_margin = 0.0;
    }
}
