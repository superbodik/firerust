use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::parse::stylesheet::Stylesheet;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_net::body::decode_text::decode_text;
use fr_net::load::load_url::load_url;
use fr_security::fetch::resource_kind::ResourceKind;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

pub fn load_linked_sheet(
    document: &Document,
    id: NodeId,
    base: &Url,
    guard: &PageGuard,
) -> Option<Stylesheet> {
    let data = document.node(id)?.kind().as_element()?;
    let is_stylesheet = data
        .attribute("rel")?
        .split_whitespace()
        .any(|part| part.eq_ignore_ascii_case("stylesheet"));
    let is_print_only = data
        .attribute("media")
        .is_some_and(|media| media.to_ascii_lowercase().contains("print"));
    if !is_stylesheet || is_print_only {
        return None;
    }
    let url = base.join(data.attribute("href")?).ok()?;
    if !guard.permit(ResourceKind::Style, &url) {
        return None;
    }
    let response = load_url(&url).ok()?;
    let text = decode_text(&response.body, response.content_type.as_deref());
    Some(parse_stylesheet(&text))
}
