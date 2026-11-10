use boa_engine::{Context, JsResult, JsValue};

use crate::natives::dom_error_to_js::dom_error_to_js;
use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;

pub fn dom_remove_child(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let child = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    document
        .borrow_mut()
        .detach(child)
        .map_err(dom_error_to_js)?;
    Ok(JsValue::undefined())
}
