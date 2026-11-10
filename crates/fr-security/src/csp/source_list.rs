use crate::csp::source_expression::SourceExpression;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceList {
    pub items: Vec<SourceExpression>,
}

impl SourceList {
    pub fn allows_inline(&self) -> bool {
        self.items.contains(&SourceExpression::UnsafeInline)
    }
}
