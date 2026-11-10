use crate::parse::declaration::Declaration;

#[derive(Debug, Clone, PartialEq)]
pub struct StyleDeclaration {
    pub declaration: Declaration,
    pub is_important: bool,
}
