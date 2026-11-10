use std::sync::{Arc, Mutex, OnceLock};

use fr_cache::lru::byte_lru_cache::ByteLruCache;
use fr_image::decode::decoded_image::DecodedImage;

pub const IMAGE_CACHE_BYTES: usize = 32 * 1024 * 1024;

type ImageCache = ByteLruCache<String, Arc<DecodedImage>>;

fn cache() -> &'static Mutex<ImageCache> {
    static CACHE: OnceLock<Mutex<ImageCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(ByteLruCache::new(IMAGE_CACHE_BYTES)))
}

pub fn cached_image(address: &str) -> Option<Arc<DecodedImage>> {
    let mut guard = cache().lock().ok()?;
    guard.get(&address.to_owned()).cloned()
}

pub fn remember_image(address: &str, image: &Arc<DecodedImage>) {
    if let Ok(mut guard) = cache().lock() {
        guard.insert(address.to_owned(), image.clone(), image.byte_len());
    }
}

pub fn shared_image_bytes() -> usize {
    cache().lock().map(|guard| guard.bytes_used()).unwrap_or(0)
}
