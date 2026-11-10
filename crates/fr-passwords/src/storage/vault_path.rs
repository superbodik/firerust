use std::path::PathBuf;

pub fn vault_path() -> Option<PathBuf> {
    let base = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(base).join("firerust").join("passwords.bin"))
}
