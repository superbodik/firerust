use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::parse::stylesheet::Stylesheet;
use fr_dom::query::element_name::element_name;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_security::fetch::resource_kind::ResourceKind;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

use crate::prefetch::resource_store::ResourceStore;
use crate::style::load_linked_sheet::load_linked_sheet;

pub fn collect_author_sheets(
    document: &Document,
    base: &Url,
    guard: &PageGuard,
    store: &ResourceStore,
) -> Vec<Stylesheet> {
    let inline_allowed = guard.allows_inline(ResourceKind::Style);
    let mut sheets: Vec<Stylesheet> = Vec::new();
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        match element_name(document, id) {
            Some("style") if inline_allowed => {
                sheets.push(parse_stylesheet(&text_content(document, id)));
            }
            Some("style") => eprintln!("blocked inline stylesheet: content security policy"),
            Some("link") => sheets.extend(load_linked_sheet(document, id, base, guard, store)),
            _ => {}
        }
    }
    sheets
}
