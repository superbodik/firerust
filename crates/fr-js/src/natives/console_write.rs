use std::cell::RefCell;

use boa_engine::{Context, JsResult, JsValue};

use crate::natives::string_arg::string_arg;

thread_local! {
    static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn console_write(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let level = string_arg(args, 0, context)?;
    let message = string_arg(args, 1, context)?;
    LINES.with(|lines| lines.borrow_mut().push(format!("[{level}] {message}")));
    Ok(JsValue::undefined())
}

pub fn take_console_lines() -> Vec<String> {
    LINES.with(|lines| std::mem::take(&mut *lines.borrow_mut()))
}
