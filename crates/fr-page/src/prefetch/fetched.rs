use fr_net::load::response::Response;

use crate::prefetch::prefetched_page::PrefetchedPage;

#[derive(Debug)]
pub enum Fetched {
    Page(PrefetchedPage),
    Download(Response),
}

impl Fetched {
    pub fn into_page(self) -> Option<PrefetchedPage> {
        match self {
            Self::Page(page) => Some(page),
            Self::Download(_) => None,
        }
    }
}
