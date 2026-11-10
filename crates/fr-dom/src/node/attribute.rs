#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    name: Box<str>,
    value: Box<str>,
}

impl Attribute {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
