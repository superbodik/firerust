use std::path::Path;

use fr_bookmarks::storage::bookmarks_path::bookmarks_path;
use fr_download::save::downloads_folder::downloads_folder;
use fr_page::load::escape_html::escape_html;
use fr_passwords::storage::vault_path::vault_path;
use fr_settings::model::search_engine::SearchEngine;
use fr_settings::model::settings::Settings;
use fr_settings::model::theme_choice::ThemeChoice;
use fr_settings::storage::settings_path::settings_path;

use crate::internal::group_html::group_html;
use crate::internal::page_html::page_html;
use crate::internal::row::Row;

fn path_text(path: Option<&Path>) -> String {
    path.map_or_else(
        || "not available".to_owned(),
        |found| escape_html(&found.display().to_string()),
    )
}

fn place(label: &str, path: Option<&Path>) -> Row {
    Row::text(&format!(
        "<span>{label}</span><span class=\"note\">{}</span>",
        path_text(path)
    ))
}

fn theme_row(choice: ThemeChoice, settings: &Settings) -> Row {
    let mark = if settings.theme == choice {
        "&#9679;"
    } else {
        ""
    };
    Row::link(
        &format!("about:settings?theme={}", choice.key()),
        &format!(
            "<span class=\"grow\">{}</span><span class=\"mark\">{mark}</span>",
            choice.label()
        ),
    )
}

fn search_row(engine: SearchEngine, settings: &Settings) -> Row {
    let mark = if settings.search == engine {
        "&#9679;"
    } else {
        ""
    };
    Row::link(
        &format!("about:settings?search={}", engine.key()),
        &format!(
            "<span class=\"grow\">{} <span class=\"note\">{}</span></span>\
             <span class=\"mark\">{mark}</span>",
            engine.label(),
            engine.hint()
        ),
    )
}

pub fn settings_page(settings: &Settings, is_theme_forced: bool, css: &str) -> String {
    let themes: Vec<Row> = ThemeChoice::ALL
        .iter()
        .map(|choice| theme_row(*choice, settings))
        .collect();
    let engines: Vec<Row> = SearchEngine::ALL
        .iter()
        .map(|engine| search_row(*engine, settings))
        .collect();
    let forced = if is_theme_forced {
        "<div class=\"callout\"><p>The FIRERUST_THEME variable is set, so it decides the \
         theme until you remove it.</p></div>"
    } else {
        ""
    };
    let places = group_html(&[
        place("Downloads", Some(downloads_folder().as_path())),
        place("Bookmarks", bookmarks_path().as_deref()),
        place("Passwords (encrypted)", vault_path().as_deref()),
        place("Settings", settings_path().as_deref()),
    ]);
    page_html(
        "Settings",
        css,
        &format!(
            "<h1>Settings</h1>\
             <p class=\"lede\">Changes apply at once and are remembered.</p>\
             <h2>Theme</h2>{}{forced}\
             <h2>Search from the address bar</h2>{}\
             <h2>Where things are kept</h2>{places}\
             <p class=\"note\"><a href=\"about:menu\">Back to the menu</a></p>",
            group_html(&themes),
            group_html(&engines)
        ),
    )
}
