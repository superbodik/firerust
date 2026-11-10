use boa_engine::{Context, JsResult, JsValue};

use crate::natives::host_document::host_document;
use crate::natives::id_value::id_value;
use crate::natives::node_arg::node_arg;

pub fn dom_parent(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    let parent = document.borrow().node(id).and_then(|node| node.parent());
    Ok(id_value(parent))
}
