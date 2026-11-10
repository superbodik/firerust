use fr_dom::node::node_id::NodeId;
use fr_form::submit::form_of::form_of;
use fr_tab::set::tab_set::TabSet;

pub fn form_for(tabs: &mut TabSet, control: NodeId) -> Option<NodeId> {
    let content = tabs.active_mut().content_mut()?;
    form_of(&content.document.borrow(), control)
}
