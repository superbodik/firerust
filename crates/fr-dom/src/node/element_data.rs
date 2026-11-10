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

    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|attribute| attribute.name() == name)
            .map(Attribute::value)
    }
}
