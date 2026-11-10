use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::cascade::compute_style::compute_style;
use crate::cascade::computed_style::ComputedStyle;
use crate::cascade::origin::Origin;
use crate::cascade::specify_style::specify_style;
use crate::cascade::style_map::StyleMap;
use crate::parse::rule::Rule;
use crate::parse::stylesheet::Stylesheet;

pub fn compute_styles(
    document: &Document,
    user_agent: &Stylesheet,
    author: &[Stylesheet],
) -> StyleMap {
    let rules: Vec<(Origin, &Rule)> = user_agent
        .rules
        .iter()
        .map(|rule| (Origin::UserAgent, rule))
        .chain(
            author
                .iter()
                .flat_map(|sheet| sheet.rules.iter())
                .map(|rule| (Origin::Author, rule)),
        )
        .collect();

    let mut styles = StyleMap::new(document.node_count());
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        let Some(data) = document.node(id).and_then(|node| node.kind().as_element()) else {
            continue;
        };
        let parent = parent_style(document, &styles, id);
        let specified = specify_style(document, id, data, &rules);
        styles.set(id, compute_style(&specified, &parent));
    }
    styles
}

fn parent_style(document: &Document, styles: &StyleMap, id: NodeId) -> ComputedStyle {
    document
        .node(id)
        .and_then(|node| node.parent())
        .and_then(|parent| styles.get(parent))
        .copied()
        .unwrap_or_else(ComputedStyle::initial)
}
