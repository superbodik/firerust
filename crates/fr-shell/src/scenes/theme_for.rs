use fr_settings::model::theme_choice::ThemeChoice;
use fr_theme::kind::theme_kind::ThemeKind;

pub fn theme_for(
    forced_by_environment: Option<ThemeKind>,
    choice: ThemeChoice,
    system_is_dark: bool,
) -> ThemeKind {
    if let Some(forced) = forced_by_environment {
        return forced;
    }
    match choice {
        ThemeChoice::System => ThemeKind::from_dark(system_is_dark),
        ThemeChoice::Light => ThemeKind::Light,
        ThemeChoice::Dark => ThemeKind::Dark,
    }
}
