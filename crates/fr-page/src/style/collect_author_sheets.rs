use fr_css::parse::parse_stylesheet::parse_stylesheet;
use fr_css::parse::stylesheet::Stylesheet;
use fr_dom::query::element_name::element_name;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use url::Url;

use crate::style::load_linked_sheet::load_linked_sheet;

pub fn collect_author_sheets(document: &Document, base: &Url) -> Vec<Stylesheet> {
    let mut sheets: Vec<Stylesheet> = Vec::new();
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        match element_name(document, id) {
            Some("style") => sheets.push(parse_stylesheet(&text_content(document, id))),
            Some("link") => sheets.extend(load_linked_sheet(document, id, base)),
            _ => {}
        }
    }
    sheets
}
