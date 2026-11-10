use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserSource {
    Chrome,
    Edge,
    Brave,
    Vivaldi,
    Opera,
}

impl BrowserSource {
    pub const ALL: [BrowserSource; 5] = [
        Self::Chrome,
        Self::Edge,
        Self::Brave,
        Self::Vivaldi,
        Self::Opera,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Self::Chrome => "chrome",
            Self::Edge => "edge",
            Self::Brave => "brave",
            Self::Vivaldi => "vivaldi",
            Self::Opera => "opera",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Chrome => "Google Chrome",
            Self::Edge => "Microsoft Edge",
            Self::Brave => "Brave",
            Self::Vivaldi => "Vivaldi",
            Self::Opera => "Opera",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|source| source.key() == key)
    }

    pub fn bookmarks_file(self) -> Option<PathBuf> {
        let local = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
        let roaming = std::env::var_os("APPDATA").map(PathBuf::from);
        match self {
            Self::Chrome => Some(local?.join(r"Google\Chrome\User Data\Default\Bookmarks")),
            Self::Edge => Some(local?.join(r"Microsoft\Edge\User Data\Default\Bookmarks")),
            Self::Brave => {
                Some(local?.join(r"BraveSoftware\Brave-Browser\User Data\Default\Bookmarks"))
            }
            Self::Vivaldi => Some(local?.join(r"Vivaldi\User Data\Default\Bookmarks")),
            Self::Opera => Some(roaming?.join(r"Opera Software\Opera Stable\Bookmarks")),
        }
    }
}
