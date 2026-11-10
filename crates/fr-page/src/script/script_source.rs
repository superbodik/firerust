use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptSource {
    Inline(String),
    External(Url),
}
