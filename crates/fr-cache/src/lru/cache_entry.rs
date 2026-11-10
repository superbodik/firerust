#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub bytes: usize,
    pub last_used: u64,
}
