use fr_dom::tree::document::Document;
use fr_geom::geometry::point::Point;
use fr_layout::hit_test::link_at::link_at;
use fr_page::link::resolve_link::resolve_link;
use fr_tab::model::layout_cache::LayoutCache;
use fr_text::engine::text_engine::TextEngine;
use url::Url;

pub fn link_target(
    layout: &LayoutCache,
    document: &Document,
    engine: &mut TextEngine,
    base: &Url,
    point: Point,
) -> Option<Url> {
    let anchor = link_at(&layout.root, engine, point)?;
    let href = document
        .node(anchor)?
        .kind()
        .as_element()?
        .attribute("href")?;
    resolve_link(base, href)
}
