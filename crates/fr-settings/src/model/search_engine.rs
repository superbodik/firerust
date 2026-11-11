use url::form_urlencoded::byte_serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchEngine {
    #[default]
    DuckDuckGo,
    Google,
    Bing,
    Yandex,
}

impl SearchEngine {
    pub const ALL: [SearchEngine; 4] = [Self::DuckDuckGo, Self::Google, Self::Bing, Self::Yandex];

    pub fn key(self) -> &'static str {
        match self {
            Self::DuckDuckGo => "duckduckgo",
            Self::Google => "google",
            Self::Bing => "bing",
            Self::Yandex => "yandex",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::DuckDuckGo => "DuckDuckGo",
            Self::Google => "Google",
            Self::Bing => "Bing",
            Self::Yandex => "Yandex",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::DuckDuckGo => "works without scripts",
            Self::Google | Self::Bing | Self::Yandex => "needs more scripts than firerust has",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|engine| engine.key().eq_ignore_ascii_case(key.trim()))
    }

    pub fn query_url(self, query: &str) -> String {
        let encoded: String = byte_serialize(query.trim().as_bytes()).collect();
        match self {
            Self::DuckDuckGo => format!("https://html.duckduckgo.com/html/?q={encoded}"),
            Self::Google => format!("https://www.google.com/search?q={encoded}"),
            Self::Bing => format!("https://www.bing.com/search?q={encoded}"),
            Self::Yandex => format!("https://yandex.com/search/?text={encoded}"),
        }
    }
}
