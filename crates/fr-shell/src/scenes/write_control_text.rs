use fr_dom::node::node_id::NodeId;
use fr_form::state::write_value::write_value;
use fr_tab::set::tab_set::TabSet;

pub fn write_control_text(tabs: &mut TabSet, control: NodeId, text: &str) -> bool {
    let Some(content) = tabs.active_mut().content_mut() else {
        return false;
    };
    write_value(&mut content.document.borrow_mut(), control, text).is_ok()
}
