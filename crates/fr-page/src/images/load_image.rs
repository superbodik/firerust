use std::sync::Arc;

use fr_image::decode::decode_image::decode_image;
use fr_image::decode::decoded_image::DecodedImage;
use url::Url;

use crate::images::shared_image_cache::{cached_image, remember_image};
use crate::prefetch::resource_store::ResourceStore;

pub fn load_image(url: &Url, store: &ResourceStore) -> Option<Arc<DecodedImage>> {
    if let Some(image) = cached_image(url.as_str()) {
        return Some(image);
    }
    let response = store.load(url)?;
    let image = Arc::new(decode_image(&response.body)?);
    remember_image(url.as_str(), &image);
    Some(image)
}
