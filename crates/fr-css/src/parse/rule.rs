use crate::parse::style_declaration::StyleDeclaration;
use crate::select::selector::Selector;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<StyleDeclaration>,
}
