use std::fs;

use fr_settings::model::search_engine::SearchEngine;
use fr_settings::model::settings::Settings;
use fr_settings::model::theme_choice::ThemeChoice;
use fr_settings::storage::decode_settings::decode_settings;
use fr_settings::storage::encode_settings::encode_settings;
use fr_settings::storage::load_settings::load_settings;
use fr_settings::storage::save_settings::save_settings;

fn scratch_file(name: &str) -> std::path::PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "firerust-settings-test-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&folder);
    folder.join("settings.json")
}

#[test]
fn the_default_follows_the_system_theme() {
    assert_eq!(Settings::default().theme, ThemeChoice::System);
}

#[test]
fn keys_and_labels_round_trip() {
    for choice in ThemeChoice::ALL {
        assert_eq!(ThemeChoice::from_key(choice.key()), Some(choice));
        assert!(!choice.label().is_empty());
    }
    assert_eq!(ThemeChoice::from_key(" Dark "), Some(ThemeChoice::Dark));
    assert_eq!(ThemeChoice::from_key("purple"), None);
}

#[test]
fn settings_survive_encoding() {
    let settings = Settings {
        theme: ThemeChoice::Dark,
        search: SearchEngine::Bing,
    };

    assert_eq!(decode_settings(&encode_settings(&settings)), Some(settings));
}

#[test]
fn an_unknown_theme_falls_back_and_garbage_is_refused() {
    assert_eq!(
        decode_settings("{\"theme\": \"neon\"}"),
        Some(Settings::default())
    );
    assert_eq!(decode_settings("not json"), None);
}

#[test]
fn saving_and_loading_use_a_file() {
    let path = scratch_file("file");
    let settings = Settings {
        theme: ThemeChoice::Light,
        search: SearchEngine::Google,
    };

    save_settings(&path, &settings).expect("saved");

    assert_eq!(load_settings(&path), settings);
    let _ = fs::remove_dir_all(path.parent().expect("parent"));
}

#[test]
fn a_missing_or_broken_file_gives_the_default() {
    let missing = scratch_file("missing");
    assert_eq!(load_settings(&missing), Settings::default());

    let broken = scratch_file("broken");
    fs::create_dir_all(broken.parent().expect("parent")).expect("folder");
    fs::write(&broken, "{{{").expect("written");
    assert_eq!(load_settings(&broken), Settings::default());
    let _ = fs::remove_dir_all(broken.parent().expect("parent"));
}

#[test]
fn the_default_search_engine_works_without_scripts() {
    assert_eq!(Settings::default().search, SearchEngine::DuckDuckGo);
}

#[test]
fn search_engine_keys_round_trip() {
    for engine in SearchEngine::ALL {
        assert_eq!(SearchEngine::from_key(engine.key()), Some(engine));
        assert!(!engine.label().is_empty());
        assert!(!engine.hint().is_empty());
    }
    assert_eq!(SearchEngine::from_key("altavista"), None);
}

#[test]
fn a_query_is_encoded_into_the_engine_address() {
    assert_eq!(
        SearchEngine::DuckDuckGo.query_url("rust lang & more"),
        "https://html.duckduckgo.com/html/?q=rust+lang+%26+more"
    );
    assert_eq!(
        SearchEngine::Google.query_url("  рубль "),
        "https://www.google.com/search?q=%D1%80%D1%83%D0%B1%D0%BB%D1%8C"
    );
    assert!(SearchEngine::Yandex.query_url("x").contains("text=x"));
    assert!(
        SearchEngine::Bing
            .query_url("x")
            .starts_with("https://www.bing.com/")
    );
}

#[test]
fn an_old_settings_file_without_a_search_key_still_loads() {
    let settings = decode_settings("{\"theme\": \"dark\"}").expect("decoded");

    assert_eq!(settings.theme, ThemeChoice::Dark);
    assert_eq!(settings.search, SearchEngine::DuckDuckGo);
}
