use boa_engine::{Context, JsResult, JsValue};
use fr_dom::tree::edge::Edge;

use crate::natives::host_document::host_document;
use crate::natives::id_value::id_value;
use crate::natives::string_arg::string_arg;

pub fn dom_get_element_by_id(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let wanted = string_arg(args, 0, context)?;
    let document = host_document(context)?;
    let document = document.borrow();
    let found = document.walk(document.root()).find_map(|edge| match edge {
        Edge::Enter(id) => {
            let element = document.node(id)?.kind().as_element()?;
            (element.attribute("id") == Some(wanted.as_str())).then_some(id)
        }
        Edge::Leave(_) => None,
    });
    Ok(id_value(found))
}
