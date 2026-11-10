use boa_engine::{Context, JsResult, JsValue};
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_html::parse::parse_html::parse_html;

use crate::natives::copy_subtree::copy_subtree;
use crate::natives::dom_error_to_js::dom_error_to_js;
use crate::natives::host_document::host_document;
use crate::natives::node_arg::node_arg;
use crate::natives::string_arg::string_arg;

pub fn dom_set_inner_html(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let id = node_arg(args, 0, context)?;
    let html = string_arg(args, 1, context)?;
    let fragment = parse_html(&html);
    let document = host_document(context)?;
    let mut document = document.borrow_mut();
    document.clear_children(id).map_err(dom_error_to_js)?;
    let Some(body) = find_body(&fragment) else {
        return Ok(JsValue::undefined());
    };
    for child in fragment.children(body) {
        let copy = copy_subtree(&fragment, child, &mut document).map_err(dom_error_to_js)?;
        document.append_child(id, copy).map_err(dom_error_to_js)?;
    }
    Ok(JsValue::undefined())
}

fn find_body(fragment: &Document) -> Option<NodeId> {
    let html = fragment
        .children(fragment.root())
        .find(|&id| element_name(fragment, id) == Some("html"))?;
    fragment
        .children(html)
        .find(|&id| element_name(fragment, id) == Some("body"))
}
