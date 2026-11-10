use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::document::Document;
use fr_image::store::image_store::ImageStore;
use url::Url;

#[derive(Debug, Clone)]
pub struct Page {
    pub url: Url,
    pub title: Option<String>,
    pub document: Document,
    pub styles: StyleMap,
    pub images: ImageStore,
}
