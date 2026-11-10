use boa_engine::{Context, JsNativeError, JsResult, JsValue};
use fr_dom::node::node_id::NodeId;

use crate::natives::host_document::host_document;

pub fn node_arg(args: &[JsValue], position: usize, context: &mut Context) -> JsResult<NodeId> {
    let number = args
        .get(position)
        .cloned()
        .unwrap_or_default()
        .to_number(context)?;
    let document = host_document(context)?;
    if number < 0.0 || number.fract() != 0.0 {
        return Err(JsNativeError::typ().with_message("not a node").into());
    }
    let id = document.borrow().id_at(number as usize);
    id.ok_or_else(|| JsNativeError::typ().with_message("not a node").into())
}
