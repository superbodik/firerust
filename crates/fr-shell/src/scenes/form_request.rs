use fr_dom::node::node_id::NodeId;
use fr_form::submit::build_request::build_request;
use fr_net::load::request::Request;
use fr_tab::set::tab_set::TabSet;

pub fn form_request(tabs: &mut TabSet, form: NodeId, submitter: Option<NodeId>) -> Option<Request> {
    let tab = tabs.active_mut();
    let base = tab.url().clone();
    let content = tab.content_mut()?;
    let document = content.document.borrow();
    build_request(&document, form, submitter, &base)
}
