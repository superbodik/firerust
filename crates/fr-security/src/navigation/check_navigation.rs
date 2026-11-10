use url::Url;

use crate::fetch::block_reason::BlockReason;

pub fn check_navigation(from: &Url, to: &Url) -> Result<(), BlockReason> {
    let to_internal = to.scheme() == "about";
    let from_internal = from.scheme() == "about";
    if to_internal && from_internal {
        return Ok(());
    }
    if !matches!(to.scheme(), "http" | "https" | "file") {
        return Err(BlockReason::UnsupportedScheme);
    }
    let from_web = matches!(from.scheme(), "http" | "https");
    if from_web && to.scheme() == "file" {
        return Err(BlockReason::LocalFileFromWeb);
    }
    Ok(())
}
