use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_image::store::image_store::ImageStore;
use fr_page::load::page::Page;

use crate::model::layout_cache::LayoutCache;

#[derive(Debug, Clone)]
pub struct TabContent {
    pub document: Document,
    pub styles: StyleMap,
    pub images: ImageStore,
    pub layout: Option<LayoutCache>,
}

impl TabContent {
    pub fn from_page(page: Page) -> Self {
        Self {
            document: page.document,
            styles: page.styles,
            images: page.images,
            layout: None,
        }
    }
}
