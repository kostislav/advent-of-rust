use std::collections::hash_map::Entry;
use std::hash::Hash;

use ahash::{HashMap, HashMapExt};

pub struct HashIndexer<T> {
    lookup: HashMap<T, usize>,
    reverse_lookup: Vec<T>,
}

impl<T: Eq + Hash + Clone> HashIndexer<T> {
    pub fn new() -> Self {
        Self { lookup: HashMap::new(), reverse_lookup: Vec::new() }
    }

    pub fn get_or_insert(&mut self, value: T) -> usize {
        let size = self.lookup.len();
        match self.lookup.entry(value) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let value = entry.key().clone();
                entry.insert(size);
                self.reverse_lookup.push(value);
                size
            }
        }
    }

    pub fn get_by_index(&self, index: usize) -> &T {
        &self.reverse_lookup[index]
    }
}

