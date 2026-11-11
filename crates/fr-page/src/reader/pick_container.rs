use std::collections::HashMap;

use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::reader::has_noise_token::has_noise_token;
use crate::reader::link_density::link_density;
use crate::reader::text_length::text_length;

const MIN_SEMANTIC_TEXT: usize = 250;
const MIN_PARAGRAPH: usize = 40;
const MAX_PARAGRAPH_WEIGHT: usize = 600;
const GOOD_TOKENS: [&str; 8] = [
    "content", "article", "post", "entry", "story", "main", "text", "body",
];

pub fn pick_container(document: &Document) -> Option<NodeId> {
    largest_semantic(document)
        .or_else(|| best_scored(document))
        .or_else(|| first_named(document, "body"))
}

fn largest_semantic(document: &Document) -> Option<NodeId> {
    let mut best: Option<(usize, NodeId)> = None;
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        if !matches!(element_name(document, id), Some("article" | "main")) {
            continue;
        }
        let length = text_length(document, id);
        let is_better = best.is_none_or(|(known, _)| length > known);
        if length >= MIN_SEMANTIC_TEXT && link_density(document, id) < 0.5 && is_better {
            best = Some((length, id));
        }
    }
    best.map(|(_, id)| id)
}

fn best_scored(document: &Document) -> Option<NodeId> {
    let mut scores: HashMap<NodeId, f32> = HashMap::new();
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        if !matches!(element_name(document, id), Some("p" | "pre" | "blockquote")) {
            continue;
        }
        let length = text_length(document, id);
        if length < MIN_PARAGRAPH {
            continue;
        }
        let weight = length.min(MAX_PARAGRAPH_WEIGHT) as f32 * (1.0 - link_density(document, id));
        let Some(parent) = parent_of(document, id) else {
            continue;
        };
        *scores.entry(parent).or_default() += weight;
        if let Some(grandparent) = parent_of(document, parent) {
            *scores.entry(grandparent).or_default() += weight / 2.0;
        }
    }
    scores
        .into_iter()
        .map(|(node, score)| (node, score * name_weight(document, node)))
        .max_by(|first, second| {
            first
                .1
                .total_cmp(&second.1)
                .then(second.0.index().cmp(&first.0.index()))
        })
        .map(|(node, _)| node)
}

fn name_weight(document: &Document, node: NodeId) -> f32 {
    let Some(element) = document
        .node(node)
        .and_then(|found| found.kind().as_element())
    else {
        return 1.0;
    };
    let class = element.attribute("class");
    let id = element.attribute("id");
    if has_noise_token(class, id) {
        return 0.3;
    }
    let is_good = [class, id].into_iter().flatten().any(|text| {
        text.split(|character: char| !character.is_alphanumeric())
            .any(|token| GOOD_TOKENS.contains(&token.to_ascii_lowercase().as_str()))
    });
    if is_good { 1.25 } else { 1.0 }
}

fn parent_of(document: &Document, node: NodeId) -> Option<NodeId> {
    document.node(node)?.parent()
}

fn first_named(document: &Document, wanted: &str) -> Option<NodeId> {
    document.walk(document.root()).find_map(|edge| match edge {
        Edge::Enter(id) if element_name(document, id) == Some(wanted) => Some(id),
        _ => None,
    })
}
