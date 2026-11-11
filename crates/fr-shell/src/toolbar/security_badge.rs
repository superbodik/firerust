use fr_geom::color::rgba::Rgba;
use fr_icons::catalog::icon::Icon;
use fr_theme::colors::palette::Palette;
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityBadge {
    Secure,
    Insecure,
    Local,
    Internal,
}

impl SecurityBadge {
    pub fn of(url: &Url) -> Self {
        match url.scheme() {
            "https" => Self::Secure,
            "http" => Self::Insecure,
            "file" => Self::Local,
            _ => Self::Internal,
        }
    }

    pub fn icon(self) -> Icon {
        match self {
            Self::Secure => Icon::LockFill,
            Self::Insecure => Icon::ExclamationTriangle,
            Self::Local => Icon::FileEarmark,
            Self::Internal => Icon::Search,
        }
    }

    pub fn color(self, palette: &Palette) -> Rgba {
        match self {
            Self::Secure => palette.secure,
            Self::Insecure => palette.warning,
            Self::Local | Self::Internal => palette.field_hint,
        }
    }
}
