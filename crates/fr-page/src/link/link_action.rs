use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkAction {
    Open { url: Url, in_new_tab: bool },
    Scroll { fragment: String },
    External { url: Url },
}

impl LinkAction {
    pub fn hover_text(&self) -> String {
        match self {
            Self::Open { url, .. } | Self::External { url } => url.to_string(),
            Self::Scroll { fragment } => format!("#{fragment}"),
        }
    }
}
