use serde_json::json;

use crate::model::settings::Settings;

pub fn encode_settings(settings: &Settings) -> String {
    let document = json!({ "version": 1, "theme": settings.theme.key() });
    serde_json::to_string_pretty(&document).unwrap_or_default()
}
