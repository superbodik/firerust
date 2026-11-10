use fr_js::clock::now_ms::now_ms;
use fr_tab::set::tab_set::TabSet;

pub fn run_due_timers(tabs: &mut TabSet) {
    if let Some(content) = tabs.active_mut().content_mut() {
        content.advance_time(now_ms());
    }
}
