use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::parse::stylesheet::Stylesheet;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_net::load::load_url::load_url;
use url::Url;

pub fn load_linked_sheet(document: &Document, id: NodeId, base: &Url) -> Option<Stylesheet> {
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
    let response = load_url(&url).ok()?;
    Some(parse_stylesheet(&String::from_utf8_lossy(&response.body)))
}
