use fr_css::cascade::compute_styles::compute_styles;
use fr_css::cascade::style_map::StyleMap;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_dom::tree::document::Document;
use fr_image::store::image_store::ImageStore;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

use crate::images::collect_images::collect_images;
use crate::prefetch::resource_store::ResourceStore;
use crate::style::collect_author_sheets::collect_author_sheets;

pub fn restyle(document: &Document, url: &Url, store: &ResourceStore) -> (StyleMap, ImageStore) {
    let guard = PageGuard::new(document, url);
    let author_sheets = collect_author_sheets(document, url, &guard, store);
    let styles = compute_styles(document, &user_agent_sheet(), &author_sheets);
    let images = collect_images(document, &styles, url, &guard, store);
    (styles, images)
}
