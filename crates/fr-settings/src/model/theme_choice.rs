#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeChoice {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeChoice {
    pub const ALL: [ThemeChoice; 3] = [Self::System, Self::Light, Self::Dark];

    pub fn key(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::System => "Follow Windows",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|choice| choice.key().eq_ignore_ascii_case(key.trim()))
    }
}
