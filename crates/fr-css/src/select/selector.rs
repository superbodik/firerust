use crate::select::selector_part::SelectorPart;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl Selector {
    pub fn specificity(&self) -> u32 {
        let mut ids: u32 = 0;
        let mut classes: u32 = 0;
        let mut tags: u32 = 0;
        for part in &self.parts {
            if part.compound.id.is_some() {
                ids += 1;
            }
            classes += (part.compound.classes.len()
                + part.compound.attributes.len()
                + part.compound.pseudos.len()) as u32;
            if part.compound.tag.is_some() {
                tags += 1;
            }
        }
        (ids.min(255) << 16) | (classes.min(255) << 8) | tags.min(255)
    }
}
