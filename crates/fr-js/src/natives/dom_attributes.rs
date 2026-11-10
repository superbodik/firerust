use boa_engine::object::builtins::JsArray;
use boa_engine::{Context, JsResult, JsString, JsValue};

use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;

pub fn dom_attributes(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    let flat: Vec<JsValue> = document
        .borrow()
        .node(id)
        .and_then(|node| node.kind().as_element())
        .map(|element| {
            element
                .attributes()
                .iter()
                .flat_map(|attribute| {
                    [
                        JsValue::from(JsString::from(attribute.name())),
                        JsValue::from(JsString::from(attribute.value())),
                    ]
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(JsArray::from_iter(flat, context).into())
}
