use url::Url;

use crate::prefetch::resource_store::ResourceStore;

#[derive(Debug)]
pub struct PrefetchedPage {
    pub html: String,
    pub url: Url,
    pub store: ResourceStore,
}
