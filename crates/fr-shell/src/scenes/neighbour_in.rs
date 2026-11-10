use fr_dom::node::node_id::NodeId;
use fr_form::state::focusable_controls::focusable_controls;
use fr_form::state::neighbour_control::neighbour_control;
use fr_tab::set::tab_set::TabSet;

pub fn neighbour_in(tabs: &mut TabSet, current: Option<NodeId>, forward: bool) -> Option<NodeId> {
    let content = tabs.active_mut().content_mut()?;
    let controls = focusable_controls(&content.document.borrow());
    neighbour_control(&controls, current, forward)
}
