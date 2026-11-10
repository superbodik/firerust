use fr_tab::set::tab_set::TabSet;

pub fn refresh_active(tabs: &mut TabSet) -> bool {
    let tab = tabs.active_mut();
    let url = tab.url().clone();
    tab.content_mut()
        .is_some_and(|content| content.refresh(&url))
}
