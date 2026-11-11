use std::collections::VecDeque;
use std::sync::Mutex;

const CAPACITY: usize = 500;

static LINES: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

pub fn record_console_line(line: &str) {
    let Ok(mut lines) = LINES.lock() else {
        return;
    };
    if lines.len() >= CAPACITY {
        lines.pop_front();
    }
    lines.push_back(line.to_owned());
}

pub fn recent_console_lines() -> Vec<String> {
    LINES
        .lock()
        .map(|lines| lines.iter().cloned().collect())
        .unwrap_or_default()
}

pub fn clear_console_lines() {
    if let Ok(mut lines) = LINES.lock() {
        lines.clear();
    }
}
