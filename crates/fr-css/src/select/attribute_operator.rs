#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeOperator {
    Exists,
    Equals,
    Includes,
    DashMatch,
    Prefix,
    Suffix,
    Substring,
}
