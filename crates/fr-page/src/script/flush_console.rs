use fr_js::runtime::js_runtime::JsRuntime;

pub fn flush_console(runtime: &mut JsRuntime) {
    for line in runtime.take_console() {
        eprintln!("js {line}");
    }
}
