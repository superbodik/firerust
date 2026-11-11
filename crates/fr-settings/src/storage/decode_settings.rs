use serde_json::Value;

use crate::model::settings::Settings;
use crate::model::theme_choice::ThemeChoice;

pub fn decode_settings(text: &str) -> Option<Settings> {
    let document: Value = serde_json::from_str(text).ok()?;
    let theme = document
        .get("theme")
        .and_then(Value::as_str)
        .and_then(ThemeChoice::from_key)
        .unwrap_or_default();
    Some(Settings { theme })
}
