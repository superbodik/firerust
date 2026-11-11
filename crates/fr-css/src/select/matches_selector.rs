use std::iter::successors;

use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::select::combinator::Combinator;
use crate::select::matches_compound::matches_compound;
use crate::select::previous_element::previous_element;
use crate::select::selector::Selector;
use crate::select::selector_part::SelectorPart;

pub fn matches_selector(document: &Document, element: NodeId, selector: &Selector) -> bool {
    matches_parts(document, element, &selector.parts)
}

fn matches_parts(document: &Document, element: NodeId, parts: &[SelectorPart]) -> bool {
    let Some((last, rest)) = parts.split_last() else {
        return true;
    };
    if !matches_compound(document, element, &last.compound) {
        return false;
    }
    if rest.is_empty() {
        return true;
    }
    match last.combinator {
        Combinator::Child => parent_element(document, element)
            .is_some_and(|parent| matches_parts(document, parent, rest)),
        Combinator::Descendant => successors(parent_element(document, element), |&ancestor| {
            parent_element(document, ancestor)
        })
        .any(|ancestor| matches_parts(document, ancestor, rest)),
        Combinator::NextSibling => previous_element(document, element)
            .is_some_and(|sibling| matches_parts(document, sibling, rest)),
        Combinator::SubsequentSibling => {
            successors(previous_element(document, element), |&sibling| {
                previous_element(document, sibling)
            })
            .any(|sibling| matches_parts(document, sibling, rest))
        }
    }
}

fn parent_element(document: &Document, id: NodeId) -> Option<NodeId> {
    let parent = document.node(id)?.parent()?;
    document.node(parent)?.kind().as_element()?;
    Some(parent)
}
