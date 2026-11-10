use fr_dom::node::element_data::ElementData;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::cascade::candidate::Candidate;
use crate::cascade::origin::Origin;
use crate::cascade::presentational_hints::presentational_hints;
use crate::cascade::specified_style::SpecifiedStyle;
use crate::parse::parse_inline_style::parse_inline_style;
use crate::parse::rule::Rule;
use crate::select::matches_selector::matches_selector;
use crate::select::selector::Selector;

const INLINE_SPECIFICITY: u32 = u32::MAX;

pub fn specify_style(
    document: &Document,
    id: NodeId,
    data: &ElementData,
    rules: &[(Origin, &Rule)],
) -> SpecifiedStyle {
    let inline = data
        .attribute("style")
        .map(parse_inline_style)
        .unwrap_or_default();
    let hints = presentational_hints(data);
    let mut candidates: Vec<Candidate<'_>> = Vec::new();
    for (index, declaration) in hints.iter().enumerate() {
        candidates.push(Candidate {
            key: (false, 0, 0, 0, index),
            declaration,
        });
    }

    for (order, (origin, rule)) in rules.iter().enumerate() {
        let Some(specificity) = rule
            .selectors
            .iter()
            .filter(|selector| matches_selector(document, id, selector))
            .map(Selector::specificity)
            .max()
        else {
            continue;
        };
        for (index, entry) in rule.declarations.iter().enumerate() {
            candidates.push(Candidate {
                key: (entry.is_important, origin.rank(), specificity, order, index),
                declaration: &entry.declaration,
            });
        }
    }
    for (index, entry) in inline.iter().enumerate() {
        candidates.push(Candidate {
            key: (
                entry.is_important,
                Origin::Author.rank(),
                INLINE_SPECIFICITY,
                usize::MAX,
                index,
            ),
            declaration: &entry.declaration,
        });
    }

    candidates.sort_by_key(|candidate| candidate.key);
    let mut specified = SpecifiedStyle::default();
    for candidate in &candidates {
        specified.apply(candidate.declaration);
    }
    specified
}
