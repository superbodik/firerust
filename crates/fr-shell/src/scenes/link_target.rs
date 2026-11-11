use fr_dom::tree::document::Document;
use fr_geom::geometry::point::Point;
use fr_layout::hit_test::link_at::link_at;
use fr_page::link::classify_link::classify_link;
use fr_page::link::link_action::LinkAction;
use fr_tab::model::layout_cache::LayoutCache;
use fr_text::engine::text_engine::TextEngine;
use url::Url;

pub fn link_target(
    layout: &LayoutCache,
    document: &Document,
    engine: &mut TextEngine,
    base: &Url,
    point: Point,
) -> Option<LinkAction> {
    let anchor = link_at(&layout.root, engine, document, point)?;
    let element = document.node(anchor)?.kind().as_element()?;
    let href = element.attribute("href")?;
    classify_link(base, href, element.attribute("target"))
}
