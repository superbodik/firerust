use url::Url;

use crate::fetch::block_reason::BlockReason;
use crate::fetch::is_trustworthy_host::is_trustworthy_host;

pub fn check_fetch(page: &Url, target: &Url) -> Result<(), BlockReason> {
    if !matches!(target.scheme(), "http" | "https" | "file") {
        return Err(BlockReason::UnsupportedScheme);
    }
    let page_is_web = matches!(page.scheme(), "http" | "https");
    if page_is_web && target.scheme() == "file" {
        return Err(BlockReason::LocalFileFromWeb);
    }
    if page.scheme() == "https" && target.scheme() == "http" && !is_trustworthy_host(target) {
        return Err(BlockReason::MixedContent);
    }
    Ok(())
}
