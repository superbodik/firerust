use fr_dom::tree::shared_document::SharedDocument;
use fr_js::log::report_script_error::report_script_error;
use fr_js::runtime::js_runtime::JsRuntime;
use fr_security::fetch::resource_kind::ResourceKind;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

use crate::prefetch::resource_store::ResourceStore;
use crate::script::collect_scripts::collect_scripts;
use crate::script::flush_console::flush_console;
use crate::script::load_script_text::load_script_text;
use crate::script::needs_scripting::needs_scripting;
use crate::script::script_source::ScriptSource;

pub fn start_scripts(
    document: &SharedDocument,
    url: &Url,
    store: &ResourceStore,
) -> Option<JsRuntime> {
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
    let guard = PageGuard::new(&document.borrow(), url);
    let inline_allowed = guard.allows_inline(ResourceKind::Script);
    if !inline_allowed && let Err(error) = runtime.forbid_inline_handlers() {
        report_script_error(&error);
    }
    let scripts = collect_scripts(&document.borrow(), url);
    for script in scripts {
        let text = match &script {
            ScriptSource::Inline(_) if !inline_allowed => {
                eprintln!("blocked inline script: content security policy");
                continue;
            }
            ScriptSource::Inline(text) => Some(text.clone()),
            ScriptSource::External(address) => {
                if !guard.permit(ResourceKind::Script, address) {
                    continue;
                }
                load_script_text(address, store)
            }
        };
        let Some(text) = text else {
            if let ScriptSource::External(address) = &script {
                eprintln!("cannot load script {address}");
            }
            continue;
        };
        if let Err(error) = runtime.run_script(&text) {
            report_script_error(&error);
        }
    }
    for event in ["DOMContentLoaded", "load"] {
        if let Err(error) = runtime.fire_event(event) {
            report_script_error(&error);
        }
    }
    flush_console(&mut runtime);
    Some(runtime)
}
