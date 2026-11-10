use boa_engine::{Context, JsResult, JsString, JsValue};
use fr_dom::query::element_name::element_name;

use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;

pub fn dom_tag_name(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    let name = element_name(&document.borrow(), id)
        .unwrap_or_default()
        .to_owned();
    Ok(JsString::from(name.as_str()).into())
}
