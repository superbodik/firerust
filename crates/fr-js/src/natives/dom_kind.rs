use boa_engine::{Context, JsResult, JsValue};
use fr_dom::node::node_kind::NodeKind;

use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;

pub fn dom_kind(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    let kind = match document.borrow().node(id).map(|node| node.kind().clone()) {
        Some(NodeKind::Element(_)) => 1.0,
        Some(NodeKind::Text(_)) => 3.0,
        Some(NodeKind::Document) => 9.0,
        None => 0.0,
    };
    Ok(JsValue::from(kind))
}
