use std::path::PathBuf;

pub fn bookmarks_path() -> Option<PathBuf> {
    let base = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(base).join("firerust").join("bookmarks.json"))
}
