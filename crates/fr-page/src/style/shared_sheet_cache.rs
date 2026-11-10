use std::sync::{Arc, Mutex, OnceLock};

use fr_cache::lru::byte_lru_cache::ByteLruCache;

pub const SHEET_CACHE_BYTES: usize = 4 * 1024 * 1024;

type SheetCache = ByteLruCache<String, Arc<str>>;

fn cache() -> &'static Mutex<SheetCache> {
    static CACHE: OnceLock<Mutex<SheetCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(ByteLruCache::new(SHEET_CACHE_BYTES)))
}

pub fn cached_sheet(address: &str) -> Option<Arc<str>> {
    let mut guard = cache().lock().ok()?;
    guard.get(&address.to_owned()).cloned()
}

pub fn remember_sheet(address: &str, text: &Arc<str>) {
    if let Ok(mut guard) = cache().lock() {
        guard.insert(address.to_owned(), text.clone(), text.len());
    }
}
