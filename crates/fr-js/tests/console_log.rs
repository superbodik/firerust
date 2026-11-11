use std::sync::Mutex;

use fr_js::log::console_log::{clear_console_lines, recent_console_lines, record_console_line};
use fr_js::log::report_script_error::report_script_error;
use fr_js::runtime::js_failure::JsFailure;

static SERIAL: Mutex<()> = Mutex::new(());

#[test]
fn recorded_lines_come_back_in_order_until_cleared() {
    let _guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_console_lines();

    record_console_line("[log] first");
    record_console_line("[warn] second");

    assert_eq!(recent_console_lines(), vec!["[log] first", "[warn] second"]);
    clear_console_lines();
    assert!(recent_console_lines().is_empty());
}

#[test]
fn the_log_keeps_only_the_newest_lines() {
    let _guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_console_lines();

    for number in 0..600 {
        record_console_line(&format!("line {number}"));
    }

    let lines = recent_console_lines();
    assert_eq!(lines.len(), 500);
    assert_eq!(lines.first().map(String::as_str), Some("line 100"));
    assert_eq!(lines.last().map(String::as_str), Some("line 599"));
    clear_console_lines();
}

#[test]
fn a_script_error_is_recorded_as_an_error_line() {
    let _guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_console_lines();

    report_script_error(&JsFailure::new("boom"));

    assert_eq!(recent_console_lines(), vec!["[error] boom"]);
    clear_console_lines();
}
