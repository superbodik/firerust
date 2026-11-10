use crate::select::attribute_condition::AttributeCondition;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Compound {
    pub tag: Option<Box<str>>,
    pub id: Option<Box<str>>,
    pub classes: Vec<Box<str>>,
    pub attributes: Vec<AttributeCondition>,
}
