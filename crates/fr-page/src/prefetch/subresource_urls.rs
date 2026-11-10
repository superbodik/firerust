use std::collections::HashSet;

use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_security::fetch::resource_kind::ResourceKind;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

use crate::images::image_url::image_url;
use crate::script::collect_scripts::collect_scripts;
use crate::script::script_source::ScriptSource;
use crate::style::linked_sheet_url::linked_sheet_url;

const MAX_IMAGES: usize = 60;

pub fn subresource_urls(document: &Document, base: &Url, guard: &PageGuard) -> Vec<Url> {
    let mut images = Vec::new();
    let mut styles = Vec::new();
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        match element_name(document, id) {
            Some("img") if images.len() < MAX_IMAGES => {
                images.extend(image_url(document, id, base));
            }
            Some("link") => styles.extend(linked_sheet_url(document, id, base)),
            _ => {}
        }
    }
    let scripts = collect_scripts(document, base)
        .into_iter()
        .filter_map(|source| match source {
            ScriptSource::External(url) => Some(url),
            ScriptSource::Inline(_) => None,
        });
    let mut seen = HashSet::new();
    let mut urls = Vec::new();
    let candidates = images
        .into_iter()
        .map(|url| (ResourceKind::Image, url))
        .chain(scripts.map(|url| (ResourceKind::Script, url)))
        .chain(styles.into_iter().map(|url| (ResourceKind::Style, url)));
    for (kind, url) in candidates {
        if guard.check(kind, &url).is_ok() && seen.insert(url.as_str().to_owned()) {
            urls.push(url);
        }
    }
    urls
}
