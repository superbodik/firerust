use boa_engine::{Context, JsResult, JsValue};
use fr_css::select::matches_selector::matches_selector;
use fr_css::select::parse_selector_list::parse_selector_list;

use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;
use crate::natives::string_arg::string_arg;

pub fn dom_matches(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let selector_text = string_arg(args, 1, context)?;
    let selectors = parse_selector_list(&selector_text);
    let document = host_document(context)?;
    let document = document.borrow();
    let is_element = document
        .node(id)
        .is_some_and(|node| node.kind().as_element().is_some());
    let matched = is_element
        && selectors
            .iter()
            .any(|selector| matches_selector(&document, id, selector));
    Ok(JsValue::from(matched))
}
