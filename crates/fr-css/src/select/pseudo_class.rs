use crate::select::compound::Compound;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    Link,
    Root,
    FirstChild,
    LastChild,
    OnlyChild,
    Empty,
    Checked,
    NthChild(i32, i32),
    NthLastChild(i32, i32),
    Not(Vec<Compound>),
}
