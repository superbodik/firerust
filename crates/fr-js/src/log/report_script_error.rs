use crate::log::console_log::record_console_line;
use crate::runtime::js_failure::JsFailure;

pub fn report_script_error(error: &JsFailure) {
    eprintln!("script error: {error}");
    record_console_line(&format!("[error] {error}"));
}
