use crate::import::browser_source::BrowserSource;
use crate::import::import_error::ImportError;
use crate::import::import_from_file::import_from_file;
use crate::model::bookmark::Bookmark;

pub fn import_from_browser(source: BrowserSource) -> Result<Vec<Bookmark>, ImportError> {
    let path = source
        .bookmarks_file()
        .ok_or_else(|| ImportError::NotFound(source.label().to_owned()))?;
    if !path.is_file() {
        return Err(ImportError::NotFound(format!(
            "the {} bookmarks file",
            source.label()
        )));
    }
    import_from_file(&path)
}
