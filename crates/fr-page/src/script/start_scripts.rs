use fr_dom::tree::shared_document::SharedDocument;
use fr_js::runtime::js_runtime::JsRuntime;
use url::Url;

use crate::script::collect_scripts::collect_scripts;
use crate::script::flush_console::flush_console;
use crate::script::load_script_text::load_script_text;
use crate::script::needs_scripting::needs_scripting;
use crate::script::script_source::ScriptSource;

pub fn start_scripts(document: &SharedDocument, url: &Url) -> Option<JsRuntime> {
    if !needs_scripting(&document.borrow()) {
        return None;
    }
    let mut runtime = match JsRuntime::new(document.clone(), url.as_str()) {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("cannot start scripts: {error}");
            return None;
        }
    };
    let scripts = collect_scripts(&document.borrow(), url);
    for script in scripts {
        let text = match &script {
            ScriptSource::Inline(text) => Some(text.clone()),
            ScriptSource::External(address) => load_script_text(address),
        };
        let Some(text) = text else {
            if let ScriptSource::External(address) = &script {
                eprintln!("cannot load script {address}");
            }
            continue;
        };
        if let Err(error) = runtime.run_script(&text) {
            eprintln!("script error: {error}");
        }
    }
    for event in ["DOMContentLoaded", "load"] {
        if let Err(error) = runtime.fire_event(event) {
            eprintln!("script error: {error}");
        }
    }
    flush_console(&mut runtime);
    Some(runtime)
}
