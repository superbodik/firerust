use std::path::Path;

use fr_bookmarks::import::browser_source::BrowserSource;
use fr_bookmarks::import::import_error::ImportError;
use fr_bookmarks::import::import_from_browser::import_from_browser;
use fr_bookmarks::import::import_from_file::import_from_file;
use fr_bookmarks::model::bookmark::Bookmark;
use fr_bookmarks::storage::bookmark_book::BookmarkBook;
use fr_page::load::blank_page::blank_page;
use fr_page::load::build_page::build_page;
use fr_page::load::error_page::error_page;
use fr_page::load::page::Page;
use fr_passwords::import::import_password_file::import_password_file;
use fr_passwords::storage::password_vault::PasswordVault;
use url::Url;

use crate::internal::bookmarks_page::bookmarks_page;
use crate::internal::import_page::import_page;
use crate::internal::import_passwords_page::{
    import_passwords_page, passwords_failed_page, passwords_imported_page,
};
use crate::internal::import_result_page::{import_failed_page, imported_page};
use crate::internal::passwords_page::passwords_page;
use crate::internal::query_value::query_value;

pub fn internal_page(
    url: &Url,
    book: &mut BookmarkBook,
    vault: &mut PasswordVault,
) -> Option<Page> {
    match url.path() {
        "blank" => blank_page(),
        "bookmarks" => {
            if let Some(address) = query_value(url, "remove") {
                book.remove(&address);
            }
            Some(build_page(&bookmarks_page(book.list()), url.clone()))
        }
        "import" => Some(build_page(&import_html(url, book), url.clone())),
        "passwords" => Some(build_page(&passwords_html(url, vault), url.clone())),
        "import-passwords" => Some(build_page(&import_passwords_html(url, vault), url.clone())),
        _ => Some(error_page(
            url.clone(),
            "This internal page does not exist.",
        )),
    }
}

fn import_html(url: &Url, book: &mut BookmarkBook) -> String {
    let outcome = match requested_import(url) {
        Some(outcome) => outcome,
        None => return import_page(),
    };
    match outcome {
        Ok(found) => {
            let count = found.len();
            let added = book.import(found);
            imported_page(count, added)
        }
        Err(error) => import_failed_page(&error.to_string()),
    }
}

fn requested_import(url: &Url) -> Option<Result<Vec<Bookmark>, ImportError>> {
    if let Some(file) = query_value(url, "file") {
        let path = file.trim().trim_matches('"').to_owned();
        return Some(import_from_file(Path::new(&path)));
    }
    let source = BrowserSource::from_key(&query_value(url, "source")?)?;
    Some(import_from_browser(source))
}

fn passwords_html(url: &Url, vault: &mut PasswordVault) -> String {
    if let (Some(origin), Some(user)) = (query_value(url, "remove"), user_value(url)) {
        let position = vault
            .list()
            .items()
            .iter()
            .position(|login| login.origin == origin && login.username == user);
        if let Some(position) = position {
            vault.remove(position);
        }
    }
    let shown = query_value(url, "show").and_then(|text| text.parse::<usize>().ok());
    passwords_page(vault.list(), shown)
}

fn user_value(url: &Url) -> Option<String> {
    url.query_pairs()
        .find(|(key, _)| key == "user")
        .map(|(_, value)| value.into_owned())
}

fn import_passwords_html(url: &Url, vault: &mut PasswordVault) -> String {
    let Some(file) = query_value(url, "file") else {
        return import_passwords_page();
    };
    let path = file.trim().trim_matches('"').to_owned();
    match import_password_file(Path::new(&path)) {
        Ok(found) => {
            let counts = vault.import(found);
            passwords_imported_page(counts.found, counts.added, counts.updated)
        }
        Err(error) => passwords_failed_page(&error.to_string()),
    }
}
