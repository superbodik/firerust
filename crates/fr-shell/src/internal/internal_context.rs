use fr_bookmarks::storage::bookmark_book::BookmarkBook;
use fr_passwords::storage::password_vault::PasswordVault;
use fr_settings::model::settings::Settings;
use fr_theme::kind::theme_kind::ThemeKind;

pub struct InternalContext<'a> {
    pub bookmarks: &'a mut BookmarkBook,
    pub vault: &'a mut PasswordVault,
    pub settings: &'a Settings,
    pub is_theme_forced: bool,
    pub theme: ThemeKind,
    pub visited: &'a [String],
}
