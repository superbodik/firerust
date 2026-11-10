use boa_engine::{Context, JsResult, JsString, JsValue};
use fr_dom::query::text_content::text_content;

use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;

pub fn dom_text_content(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let document = host_document(context)?;
    let text = text_content(&document.borrow(), id);
    Ok(JsString::from(text.as_str()).into())
}
