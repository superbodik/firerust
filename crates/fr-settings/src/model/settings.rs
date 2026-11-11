use crate::model::theme_choice::ThemeChoice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Settings {
    pub theme: ThemeChoice,
}
