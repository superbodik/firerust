use std::path::Path;

use fr_dpapi::unprotect::unprotect;

use crate::model::password_list::PasswordList;
use crate::storage::decode_vault::decode_vault;
use crate::storage::vault_entropy::VAULT_ENTROPY;

pub fn load_vault(path: &Path) -> PasswordList {
    let Ok(sealed) = std::fs::read(path) else {
        return PasswordList::new();
    };
    let Some(plain) = unprotect(&sealed, VAULT_ENTROPY) else {
        eprintln!("cannot open the password file {}", path.display());
        return PasswordList::new();
    };
    decode_vault(&plain).unwrap_or_default()
}
