use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::csp::content_policy::ContentPolicy;
use crate::csp::parse_policy::parse_policy;

pub fn policies_from_document(document: &Document) -> Vec<ContentPolicy> {
    let mut policies = Vec::new();
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        let Some(element) = document.node(id).and_then(|node| node.kind().as_element()) else {
            continue;
        };
        if element.name() != "meta" {
            continue;
        }
        let is_policy = element
            .attribute("http-equiv")
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("content-security-policy"));
        if !is_policy {
            continue;
        }
        if let Some(content) = element.attribute("content") {
            policies.push(parse_policy(content));
        }
    }
    policies
}
