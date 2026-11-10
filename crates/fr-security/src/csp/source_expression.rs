#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceExpression {
    SelfOrigin,
    UnsafeInline,
    Any,
    Scheme(String),
    Host {
        scheme: Option<String>,
        host: String,
        port: Option<u16>,
        subdomains: bool,
    },
}
