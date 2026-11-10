use std::collections::HashMap;
use std::sync::Arc;

use fr_css::cascade::style_map::StyleMap;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_image::decode::decoded_image::DecodedImage;
use fr_image::store::image_store::ImageStore;
use url::Url;

use crate::images::load_image::load_image;

const MAX_DOWNLOADS: usize = 60;

pub fn collect_images(document: &Document, styles: &StyleMap, base: &Url) -> ImageStore {
    let mut store = ImageStore::new();
    let mut cache: HashMap<String, Option<Arc<DecodedImage>>> = HashMap::new();
    let mut hidden_root: Option<NodeId> = None;

    for edge in document.walk(document.root()) {
        if let Some(root) = hidden_root {
            if edge == Edge::Leave(root) {
                hidden_root = None;
            }
            continue;
        }
        let Edge::Enter(id) = edge else {
            continue;
        };
        if styles
            .get(id)
            .is_some_and(|style| style.display == Display::None)
        {
            hidden_root = Some(id);
            continue;
        }
        if element_name(document, id) != Some("img") {
            continue;
        }
        let Some(url) = image_url(document, id, base) else {
            continue;
        };
        let key = url.as_str().to_owned();
        if !cache.contains_key(&key) {
            let image = if cache.len() < MAX_DOWNLOADS {
                load_image(&url)
            } else {
                None
            };
            cache.insert(key.clone(), image);
        }
        if let Some(Some(image)) = cache.get(&key) {
            store.insert(id, image.clone());
        }
    }
    store
}

fn image_url(document: &Document, id: NodeId, base: &Url) -> Option<Url> {
    let source = document
        .node(id)?
        .kind()
        .as_element()?
        .attribute("src")?
        .trim();
    if source.is_empty() {
        return None;
    }
    base.join(source).ok()
}
