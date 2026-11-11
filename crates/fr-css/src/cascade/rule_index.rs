use std::collections::HashMap;

use fr_dom::node::element_data::ElementData;

use crate::cascade::origin::Origin;
use crate::parse::rule::Rule;
use crate::select::selector::Selector;

#[derive(Debug, Default)]
pub struct RuleIndex {
    by_id: HashMap<Box<str>, Vec<usize>>,
    by_class: HashMap<Box<str>, Vec<usize>>,
    by_tag: HashMap<Box<str>, Vec<usize>>,
    universal: Vec<usize>,
}

impl RuleIndex {
    pub fn build(rules: &[(Origin, &Rule)]) -> Self {
        let mut index = Self::default();
        for (order, (_, rule)) in rules.iter().enumerate() {
            for selector in &rule.selectors {
                index.add(order, selector);
            }
        }
        index
    }

    pub fn candidates(&self, data: &ElementData) -> Vec<usize> {
        let mut found: Vec<usize> = self.universal.clone();
        if let Some(bucket) = self.by_tag.get(data.name()) {
            found.extend_from_slice(bucket);
        }
        if let Some(bucket) = data.attribute("id").and_then(|id| self.by_id.get(id)) {
            found.extend_from_slice(bucket);
        }
        if let Some(classes) = data.attribute("class") {
            for class in classes.split_whitespace() {
                if let Some(bucket) = self.by_class.get(class) {
                    found.extend_from_slice(bucket);
                }
            }
        }
        found.sort_unstable();
        found.dedup();
        found
    }

    fn add(&mut self, order: usize, selector: &Selector) {
        let Some(last) = selector.parts.last() else {
            return;
        };
        let compound = &last.compound;
        if let Some(id) = &compound.id {
            self.by_id.entry(id.clone()).or_default().push(order);
        } else if let Some(class) = compound.classes.first() {
            self.by_class.entry(class.clone()).or_default().push(order);
        } else if let Some(tag) = &compound.tag {
            self.by_tag.entry(tag.clone()).or_default().push(order);
        } else {
            self.universal.push(order);
        }
    }
}
