use fr_form::login::fill_login::fill_login;
use fr_form::login::find_login_fields::find_login_fields;
use fr_passwords::model::origin_of::origin_of;
use fr_passwords::storage::password_vault::PasswordVault;
use fr_tab::set::tab_set::TabSet;

pub fn autofill_logins(tabs: &mut TabSet, vault: &PasswordVault) -> bool {
    let tab = tabs.active_mut();
    let Some(origin) = origin_of(tab.url().as_str()) else {
        return false;
    };
    let Some(saved) = vault
        .matching(&origin)
        .first()
        .map(|login| (*login).clone())
    else {
        return false;
    };
    let Some(content) = tab.content_mut() else {
        return false;
    };
    let mut document = content.document.borrow_mut();
    let mut filled = false;
    for fields in find_login_fields(&document) {
        filled |= fill_login(&mut document, &fields, &saved.username, &saved.password);
    }
    filled
}
