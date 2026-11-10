use fr_net::body::decode_text::decode_text;
use url::Url;

use crate::prefetch::resource_store::ResourceStore;

pub fn load_script_text(url: &Url, store: &ResourceStore) -> Option<String> {
    let response = store.load(url)?;
    Some(decode_text(
        &response.body,
        response.content_type.as_deref(),
    ))
}
