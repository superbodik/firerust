use boa_engine::{Context, JsResult, JsString, JsValue};

use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;
use crate::natives::string_arg::string_arg;

pub fn dom_get_attribute(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let name = string_arg(args, 1, context)?;
    let document = host_document(context)?;
    let value = document
        .borrow()
        .node(id)
        .and_then(|node| node.kind().as_element())
        .and_then(|element| element.attribute(&name).map(str::to_owned));
    Ok(match value {
        Some(text) => JsString::from(text.as_str()).into(),
        None => JsValue::null(),
    })
}
