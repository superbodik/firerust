use boa_engine::object::builtins::JsArray;
use boa_engine::{Context, JsResult, JsValue};

use crate::natives::host_document::host_document;
use crate::natives::id_value::id_value;
use crate::natives::node_arg::node_arg;

pub fn dom_child_nodes(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    let children: Vec<JsValue> = document
        .borrow()
        .children(id)
        .map(|child| id_value(Some(child)))
        .collect();
    Ok(JsArray::from_iter(children, context).into())
}
