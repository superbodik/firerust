use boa_engine::object::builtins::JsArray;
use boa_engine::{Context, JsResult, JsValue};
use fr_css::select::matches_selector::matches_selector;
use fr_css::select::parse_selector_list::parse_selector_list;
use fr_dom::tree::edge::Edge;

use crate::natives::host_document::host_document;
use crate::natives::id_value::id_value;
use crate::natives::node_arg::node_arg;
use crate::natives::string_arg::string_arg;

pub fn dom_query_all(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let scope = node_arg(args, 0, context)?;
    let selector_text = string_arg(args, 1, context)?;
    let selectors = parse_selector_list(&selector_text);
    let document = host_document(context)?;
    let document = document.borrow();
    let matches: Vec<JsValue> = document
        .walk(scope)
        .filter_map(|edge| match edge {
            Edge::Enter(id) if id != scope => Some(id),
            _ => None,
        })
        .filter(|&id| {
            document
                .node(id)
                .is_some_and(|node| node.kind().as_element().is_some())
                && selectors
                    .iter()
                    .any(|selector| matches_selector(&document, id, selector))
        })
        .map(|id| id_value(Some(id)))
        .collect();
    Ok(JsArray::from_iter(matches, context).into())
}
