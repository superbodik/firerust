use boa_engine::{Context, JsResult, JsValue};

use crate::natives::dom_error_to_js::dom_error_to_js;
use crate::natives::host_document::host_document;
use crate::natives::id_value::id_value;
use crate::natives::string_arg::string_arg;

pub fn dom_create_text(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let text = string_arg(args, 0, context)?;
    let document = host_document(context)?;
    let id = document
        .borrow_mut()
        .create_text(&text)
        .map_err(dom_error_to_js)?;
    Ok(id_value(Some(id)))
}
