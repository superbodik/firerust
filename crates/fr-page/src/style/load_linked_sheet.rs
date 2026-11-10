use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::parse::stylesheet::Stylesheet;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_security::fetch::resource_kind::ResourceKind;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

use crate::prefetch::resource_store::ResourceStore;
use crate::style::linked_sheet_url::linked_sheet_url;
use crate::style::sheet_text::sheet_text;

pub fn load_linked_sheet(
    document: &Document,
    id: NodeId,
    base: &Url,
    guard: &PageGuard,
    store: &ResourceStore,
) -> Option<Stylesheet> {
    let url = linked_sheet_url(document, id, base)?;
    if !guard.permit(ResourceKind::Style, &url) {
        return None;
    }
    let text = sheet_text(&url, store)?;
    Some(parse_stylesheet(&text))
}
