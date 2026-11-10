use crate::node::element_data::ElementData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Document,
    Element(ElementData),
    Text(Box<str>),
}

impl NodeKind {
    pub fn as_element(&self) -> Option<&ElementData> {
        match self {
            Self::Element(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_element_mut(&mut self) -> Option<&mut ElementData> {
        match self {
            Self::Element(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }
}
