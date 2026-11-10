use crate::parse::rule::Rule;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}
