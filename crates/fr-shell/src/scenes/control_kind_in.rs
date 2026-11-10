use fr_dom::node::node_id::NodeId;
use fr_layout::control::control_kind::ControlKind;
use fr_layout::control::control_kind_of::control_kind_of;
use fr_tab::set::tab_set::TabSet;

pub fn control_kind_in(tabs: &mut TabSet, control: NodeId) -> Option<ControlKind> {
    let content = tabs.active_mut().content_mut()?;
    control_kind_of(&content.document.borrow(), control)
}
