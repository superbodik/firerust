use crate::csp::source_list::SourceList;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContentPolicy {
    pub default_src: Option<SourceList>,
    pub script_src: Option<SourceList>,
    pub style_src: Option<SourceList>,
    pub img_src: Option<SourceList>,
}
