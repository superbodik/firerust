use fr_dom::node::node_id::NodeId;
use fr_form::login::read_login::read_login;
use fr_passwords::model::credential::Credential;
use fr_passwords::model::origin_of::origin_of;
use fr_passwords::storage::password_vault::PasswordVault;
use fr_tab::set::tab_set::TabSet;

pub fn login_to_save(tabs: &mut TabSet, vault: &PasswordVault, form: NodeId) -> Option<Credential> {
    let tab = tabs.active_mut();
    let origin = origin_of(tab.url().as_str())?;
    let content = tab.content()?;
    let (username, password) = read_login(&content.document.borrow(), form)?;
    let known = vault
        .matching(&origin)
        .iter()
        .any(|login| login.username == username && login.password == password);
    if known {
        return None;
    }
    Some(Credential::new(&origin, &username, &password))
}
