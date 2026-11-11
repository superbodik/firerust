use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::select::index_from_end::index_from_end;
use crate::select::index_from_start::index_from_start;
use crate::select::matches_compound::matches_compound;
use crate::select::next_element::next_element;
use crate::select::nth_matches::nth_matches;
use crate::select::previous_element::previous_element;
use crate::select::pseudo_class::PseudoClass;

pub fn matches_pseudo(document: &Document, element: NodeId, pseudo: &PseudoClass) -> bool {
    let Some(node) = document.node(element) else {
        return false;
    };
    let Some(data) = node.kind().as_element() else {
        return false;
    };
    match pseudo {
        PseudoClass::Link => {
            matches!(data.name(), "a" | "area") && data.attribute("href").is_some()
        }
        PseudoClass::Root => {
            let parent_is_element = node
                .parent()
                .and_then(|parent| document.node(parent))
                .is_some_and(|parent| parent.kind().as_element().is_some());
            !parent_is_element
        }
        PseudoClass::FirstChild => previous_element(document, element).is_none(),
        PseudoClass::LastChild => next_element(document, element).is_none(),
        PseudoClass::OnlyChild => {
            previous_element(document, element).is_none()
                && next_element(document, element).is_none()
        }
        PseudoClass::Empty => document.children(element).all(|child| {
            document
                .node(child)
                .and_then(|inner| inner.kind().as_text())
                .is_some_and(|text| text.is_empty())
        }),
        PseudoClass::Checked => {
            data.attribute("checked").is_some() || data.attribute("selected").is_some()
        }
        PseudoClass::NthChild(step, offset) => {
            nth_matches(*step, *offset, index_from_start(document, element))
        }
        PseudoClass::NthLastChild(step, offset) => {
            nth_matches(*step, *offset, index_from_end(document, element))
        }
        PseudoClass::Not(compounds) => !compounds
            .iter()
            .any(|compound| matches_compound(document, element, compound)),
    }
}
