use fr_css::value::float_side::FloatSide;

use crate::flow::flow_state::FlowState;
use crate::metrics::content_area::ContentArea;

pub fn narrowed_area(state: &FlowState, at: f32) -> ContentArea {
    let mut x = state.area.x;
    let mut width = state.area.width;
    for band in state.floats.iter().filter(|band| band.bottom > at) {
        match band.side {
            FloatSide::Left => {
                x += band.width;
                width -= band.width;
            }
            FloatSide::Right => width -= band.width,
            FloatSide::None => {}
        }
    }
    ContentArea {
        x,
        y: state.area.y,
        width: width.max(0.0),
    }
}
