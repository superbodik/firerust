use crate::select::combinator::Combinator;
use crate::select::compound::Compound;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorPart {
    pub combinator: Combinator,
    pub compound: Compound,
}
