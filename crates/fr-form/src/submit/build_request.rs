use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_net::load::request::Request;
use url::Url;

use crate::state::attribute_of::attribute_of;
use crate::submit::collect_fields::collect_fields;
use crate::submit::encode_form::encode_form;

const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

pub fn build_request(
    document: &Document,
    form: NodeId,
    submitter: Option<NodeId>,
    base: &Url,
) -> Option<Request> {
    let fields = collect_fields(document, form, submitter);
    let encoded = encode_form(&fields);
    let action = attribute_of(document, form, "action").unwrap_or_default();
    let mut target = if action.trim().is_empty() {
        base.clone()
    } else {
        base.join(action.trim()).ok()?
    };
    let is_post = attribute_of(document, form, "method")
        .is_some_and(|method| method.trim().eq_ignore_ascii_case("post"));
    if is_post {
        target.set_fragment(None);
        return Some(Request::post(
            target,
            encoded.into_bytes(),
            FORM_CONTENT_TYPE,
        ));
    }
    target.set_fragment(None);
    target.set_query(if encoded.is_empty() {
        None
    } else {
        Some(&encoded)
    });
    Some(Request::get(target))
}
