use fr_css::cascade::style_map::StyleMap;
use fr_dom::tree::shared_document::SharedDocument;
use fr_image::store::image_store::ImageStore;
use fr_js::runtime::js_runtime::JsRuntime;
use url::Url;

#[derive(Debug)]
pub struct Page {
    pub url: Url,
    pub title: Option<String>,
    pub document: SharedDocument,
    pub styles: StyleMap,
    pub images: ImageStore,
    pub runtime: Option<JsRuntime>,
}
