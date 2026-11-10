use fr_dom::node::node_id::NodeId;
use fr_form::state::read_value::read_value;
use fr_tab::set::tab_set::TabSet;

pub fn read_control_text(tabs: &mut TabSet, control: NodeId) -> String {
    tabs.active_mut()
        .content_mut()
        .map(|content| read_value(&content.document.borrow(), control))
        .unwrap_or_default()
}
