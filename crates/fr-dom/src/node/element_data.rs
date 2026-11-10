use crate::node::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementData {
    name: Box<str>,
    attributes: Vec<Attribute>,
}

impl ElementData {
    pub fn new(name: &str, attributes: Vec<Attribute>) -> Self {
        Self {
            name: name.into(),
            attributes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        match self
            .attributes
            .iter_mut()
            .find(|attribute| attribute.name() == name)
        {
            Some(existing) => *existing = Attribute::new(name, value),
            None => self.attributes.push(Attribute::new(name, value)),
        }
    }

    pub fn remove_attribute(&mut self, name: &str) -> bool {
        let before = self.attributes.len();
        self.attributes.retain(|attribute| attribute.name() != name);
        self.attributes.len() != before
    }

    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|attribute| attribute.name() == name)
            .map(Attribute::value)
    }
}
