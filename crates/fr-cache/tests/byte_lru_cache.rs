use fr_cache::lru::byte_lru_cache::ByteLruCache;

#[test]
fn a_stored_value_can_be_read_back() {
    let mut cache = ByteLruCache::new(100);

    assert!(cache.insert("a", 1, 10));

    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.bytes_used(), 10);
    assert_eq!(cache.len(), 1);
}

#[test]
fn a_missing_key_gives_nothing() {
    let mut cache: ByteLruCache<&str, i32> = ByteLruCache::new(100);

    assert_eq!(cache.get(&"missing"), None);
    assert!(cache.is_empty());
}

#[test]
fn the_oldest_entry_is_evicted_when_the_limit_is_exceeded() {
    let mut cache = ByteLruCache::new(30);
    cache.insert("a", 1, 10);
    cache.insert("b", 2, 10);
    cache.insert("c", 3, 10);

    cache.insert("d", 4, 10);

    assert!(!cache.contains(&"a"));
    assert!(cache.contains(&"b"));
    assert!(cache.contains(&"d"));
    assert_eq!(cache.bytes_used(), 30);
}

#[test]
fn reading_an_entry_protects_it_from_eviction() {
    let mut cache = ByteLruCache::new(30);
    cache.insert("a", 1, 10);
    cache.insert("b", 2, 10);
    cache.insert("c", 3, 10);
    cache.get(&"a");

    cache.insert("d", 4, 10);

    assert!(cache.contains(&"a"));
    assert!(!cache.contains(&"b"));
}

#[test]
fn a_big_entry_evicts_several_small_ones() {
    let mut cache = ByteLruCache::new(30);
    cache.insert("a", 1, 10);
    cache.insert("b", 2, 10);
    cache.insert("c", 3, 10);

    cache.insert("big", 9, 25);

    assert_eq!(cache.len(), 1);
    assert!(cache.contains(&"big"));
    assert_eq!(cache.bytes_used(), 25);
}

#[test]
fn an_entry_larger_than_the_limit_is_refused() {
    let mut cache = ByteLruCache::new(30);
    cache.insert("a", 1, 10);

    assert!(!cache.insert("huge", 9, 31));

    assert!(!cache.contains(&"huge"));
    assert!(cache.contains(&"a"));
}

#[test]
fn replacing_a_key_updates_the_byte_count() {
    let mut cache = ByteLruCache::new(100);
    cache.insert("a", 1, 40);

    cache.insert("a", 2, 10);

    assert_eq!(cache.get(&"a"), Some(&2));
    assert_eq!(cache.bytes_used(), 10);
    assert_eq!(cache.len(), 1);
}

#[test]
fn removing_frees_the_bytes() {
    let mut cache = ByteLruCache::new(100);
    cache.insert("a", 1, 40);

    assert_eq!(cache.remove(&"a"), Some(1));

    assert_eq!(cache.bytes_used(), 0);
    assert!(cache.is_empty());
}

#[test]
fn clearing_empties_the_cache() {
    let mut cache = ByteLruCache::new(100);
    cache.insert("a", 1, 40);
    cache.insert("b", 2, 40);

    cache.clear();

    assert!(cache.is_empty());
    assert_eq!(cache.bytes_used(), 0);
}

#[test]
fn a_zero_limit_stores_nothing() {
    let mut cache = ByteLruCache::new(0);

    assert!(!cache.insert("a", 1, 1));
    assert!(cache.is_empty());
}
