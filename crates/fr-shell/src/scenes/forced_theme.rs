use fr_theme::kind::theme_kind::ThemeKind;

pub fn forced_theme() -> Option<ThemeKind> {
    let value = std::env::var("FIRERUST_THEME").ok()?;
    match value.trim().to_ascii_lowercase().as_str() {
        "dark" => Some(ThemeKind::Dark),
        "light" => Some(ThemeKind::Light),
        _ => None,
    }
}
