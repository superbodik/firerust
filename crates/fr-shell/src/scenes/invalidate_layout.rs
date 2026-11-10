use fr_tab::set::tab_set::TabSet;

pub fn invalidate_layout(tabs: &mut TabSet) {
    if let Some(content) = tabs.active_mut().content_mut() {
        content.layout = None;
    }
}
