use crate::select::attribute_operator::AttributeOperator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeCondition {
    pub name: Box<str>,
    pub operator: AttributeOperator,
    pub value: Box<str>,
    pub ignores_case: bool,
}
