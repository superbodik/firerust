use std::rc::Rc;

use fr_dom::node::element_data::ElementData;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::cascade::candidate::Candidate;
use crate::cascade::custom_properties::CustomProperties;
use crate::cascade::origin::Origin;
use crate::cascade::presentational_hints::presentational_hints;
use crate::cascade::rule_index::RuleIndex;
use crate::cascade::specified_style::SpecifiedStyle;
use crate::cascade::substitute_vars::substitute_vars;
use crate::parse::declaration::Declaration;
use crate::parse::parse_inline_style::parse_inline_style;
use crate::parse::parse_property_text::parse_property_text;
use crate::parse::rule::Rule;
use crate::select::matches_selector::matches_selector;
use crate::select::selector::Selector;

const INLINE_SPECIFICITY: u32 = u32::MAX;

pub fn specify_style(
    document: &Document,
    id: NodeId,
    data: &ElementData,
    rules: &[(Origin, &Rule)],
    index: &RuleIndex,
    inherited: &Rc<CustomProperties>,
) -> (SpecifiedStyle, Rc<CustomProperties>) {
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

    for order in index.candidates(data) {
        let (origin, rule) = &rules[order];
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
    let vars = collect_custom_properties(&candidates, inherited);
    let mut specified = SpecifiedStyle::default();
    for candidate in &candidates {
        match candidate.declaration {
            Declaration::Custom(..) => {}
            Declaration::Deferred(property, raw) => {
                let Some(text) = substitute_vars(raw, &vars) else {
                    continue;
                };
                for declaration in parse_property_text(property, &text).unwrap_or_default() {
                    specified.apply(&declaration);
                }
            }
            other => specified.apply(other),
        }
    }
    (specified, vars)
}

fn collect_custom_properties(
    candidates: &[Candidate<'_>],
    inherited: &Rc<CustomProperties>,
) -> Rc<CustomProperties> {
    let mut own: Option<CustomProperties> = None;
    for candidate in candidates {
        let Declaration::Custom(name, value) = candidate.declaration else {
            continue;
        };
        let map = own.get_or_insert_with(|| (**inherited).clone());
        match substitute_vars(value, map) {
            Some(resolved) => {
                map.insert(name.clone(), resolved.into());
            }
            None => {
                map.remove(name);
            }
        }
    }
    own.map_or_else(|| Rc::clone(inherited), Rc::new)
}
