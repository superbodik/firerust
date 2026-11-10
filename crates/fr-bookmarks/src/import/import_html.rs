use fr_dom::query::element_name::element_name;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;

use crate::model::bookmark::Bookmark;
use crate::model::is_bookmarkable::is_bookmarkable;

pub fn import_html(text: &str) -> Vec<Bookmark> {
    let document = parse_html(text);
    let mut found = Vec::new();
    let mut folders: Vec<String> = Vec::new();
    let mut next_folder: Option<String> = None;
    for edge in document.walk(document.root()) {
        match edge {
            Edge::Enter(id) => match element_name(&document, id) {
                Some("h3") => next_folder = Some(text_content(&document, id).trim().to_owned()),
                Some("dl") => folders.push(next_folder.take().unwrap_or_default()),
                Some("a") => {
                    let Some(element) = document.node(id).and_then(|node| node.kind().as_element())
                    else {
                        continue;
                    };
                    let Some(url) = element.attribute("href") else {
                        continue;
                    };
                    if !is_bookmarkable(url) {
                        continue;
                    }
                    let title = text_content(&document, id);
                    let folder = folders
                        .iter()
                        .filter(|name| !name.is_empty())
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" / ");
                    found.push(Bookmark::new(&title, url, &folder));
                }
                _ => {}
            },
            Edge::Leave(id) => {
                if element_name(&document, id) == Some("dl") {
                    folders.pop();
                }
            }
        }
    }
    found
}
