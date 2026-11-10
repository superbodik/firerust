use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use url::Url;

use crate::script::is_javascript_type::is_javascript_type;
use crate::script::script_source::ScriptSource;

pub fn collect_scripts(document: &Document, base: &Url) -> Vec<ScriptSource> {
    let mut scripts = Vec::new();
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        let Some(element) = document.node(id).and_then(|node| node.kind().as_element()) else {
            continue;
        };
        if element.name() != "script"
            || element.attribute("nomodule").is_some()
            || !is_javascript_type(element.attribute("type"))
        {
            continue;
        }
        match element.attribute("src").map(str::trim) {
            Some(source) if !source.is_empty() => {
                if let Ok(url) = base.join(source) {
                    scripts.push(ScriptSource::External(url));
                }
            }
            _ => {
                let text = text_content(document, id);
                if !text.trim().is_empty() {
                    scripts.push(ScriptSource::Inline(text));
                }
            }
        }
    }
    scripts
}
