use std::collections::HashMap;
use std::hash::Hash;

use crate::lru::cache_entry::CacheEntry;

#[derive(Debug, Clone)]
pub struct ByteLruCache<K, V> {
    entries: HashMap<K, CacheEntry<V>>,
    byte_limit: usize,
    bytes_used: usize,
    clock: u64,
}

impl<K: Hash + Eq + Clone, V> ByteLruCache<K, V> {
    pub fn new(byte_limit: usize) -> Self {
        Self {
            entries: HashMap::new(),
            byte_limit,
            bytes_used: 0,
            clock: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn bytes_used(&self) -> usize {
        self.bytes_used
    }

    pub fn byte_limit(&self) -> usize {
        self.byte_limit
    }

    pub fn contains(&self, key: &K) -> bool {
        self.entries.contains_key(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.clock += 1;
        let clock = self.clock;
        let entry = self.entries.get_mut(key)?;
        entry.last_used = clock;
        Some(&entry.value)
    }

    pub fn insert(&mut self, key: K, value: V, bytes: usize) -> bool {
        self.remove(&key);
        if bytes > self.byte_limit {
            return false;
        }
        while self.bytes_used + bytes > self.byte_limit {
            if !self.evict_oldest() {
                break;
            }
        }
        self.clock += 1;
        self.bytes_used += bytes;
        self.entries.insert(
            key,
            CacheEntry {
                value,
                bytes,
                last_used: self.clock,
            },
        );
        true
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let entry = self.entries.remove(key)?;
        self.bytes_used -= entry.bytes;
        Some(entry.value)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes_used = 0;
    }

    fn evict_oldest(&mut self) -> bool {
        let oldest = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_used)
            .map(|(key, _)| key.clone());
        match oldest {
            Some(key) => self.remove(&key).is_some(),
            None => false,
        }
    }
}
