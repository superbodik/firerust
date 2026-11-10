use std::io;
use std::path::Path;

use fr_dpapi::protect::protect;

use crate::model::password_list::PasswordList;
use crate::storage::encode_vault::encode_vault;
use crate::storage::vault_entropy::VAULT_ENTROPY;

pub fn save_vault(path: &Path, list: &PasswordList) -> io::Result<()> {
    let sealed = protect(&encode_vault(list), VAULT_ENTROPY).ok_or_else(|| {
        io::Error::other("the passwords could not be encrypted with the Windows account key")
    })?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("bin.tmp");
    std::fs::write(&temporary, sealed)?;
    std::fs::rename(&temporary, path)
}
