use fr_dom::node::node_id::NodeId;
use fr_tab::set::tab_set::TabSet;

pub fn dispatch_event_to(tabs: &mut TabSet, node: NodeId, event_type: &str) -> bool {
    tabs.active_mut()
        .content_mut()
        .is_none_or(|content| content.dispatch_event(node, event_type, None))
}
