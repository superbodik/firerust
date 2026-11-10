use std::time::Duration;

use fr_js::clock::now_ms::now_ms;
use fr_tab::set::tab_set::TabSet;

const SHORTEST_DELAY_MS: f64 = 4.0;

pub fn timer_delay(tabs: &mut TabSet) -> Option<Duration> {
    let due = tabs.active_mut().content_mut()?.next_timer_due()?;
    let wait = (due - now_ms()).max(SHORTEST_DELAY_MS);
    Some(Duration::from_secs_f64(wait / 1000.0))
}
