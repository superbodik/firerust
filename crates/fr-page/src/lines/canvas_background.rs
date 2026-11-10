use fr_css::cascade::style_map::StyleMap;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_geom::color::rgba::Rgba;

pub fn canvas_background(document: &Document, styles: &StyleMap) -> Rgba {
    ["html", "body"]
        .iter()
        .find_map(|name| background_of(document, styles, name))
        .unwrap_or(Rgba::WHITE)
}

fn background_of(document: &Document, styles: &StyleMap, name: &str) -> Option<Rgba> {
    let id: NodeId = document.walk(document.root()).find_map(|edge| match edge {
        Edge::Enter(id) if element_name(document, id) == Some(name) => Some(id),
        _ => None,
    })?;
    let color = styles.get(id)?.background_color;
    (color.a > 0).then_some(color)
}
