use boa_engine::{Context, JsResult, JsValue};

use crate::natives::dom_error_to_js::dom_error_to_js;
use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;

pub fn dom_insert_before(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let parent = node_arg(args, 0, context)?;
    let child = node_arg(args, 1, context)?;
    let reference_number = args
        .get(2)
        .cloned()
        .unwrap_or_default()
        .to_number(context)?;
    let reference = if reference_number < 0.0 {
        None
    } else {
        Some(node_arg(args, 2, context)?)
    };
    let document = host_document(context)?;
    let mut document = document.borrow_mut();
    document.detach(child).map_err(dom_error_to_js)?;
    document
        .insert_before(parent, child, reference)
        .map_err(dom_error_to_js)?;
    Ok(JsValue::undefined())
}
