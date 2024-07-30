use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::hash::Hash;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use derive_new::new;

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


pub fn shortest_path<T, I, F>(starting_node: T, target_node: T, edge_supplier: F) -> usize
    where T: Eq + Hash + Copy, I: Iterator<Item=(T, usize)>, F: Fn(T) -> I {
    let mut to_visit = BinaryHeap::new();
    let mut visited: HashSet<T> = HashSet::new();
    to_visit.push(Neighbor::new(starting_node, 0));

    loop {
        let closest = to_visit.pop().unwrap();
        if closest.node == target_node {
            return closest.weight;
        } else {
            if !visited.contains(&closest.node) {
                visited.insert(closest.node);
                for (neighbor, weight) in edge_supplier(closest.node) {
                    if !visited.contains(&neighbor) {
                        to_visit.push(Neighbor::new(neighbor, closest.weight + weight));
                    }
                }
            }
        }
    }
}

#[derive(new)]
struct Neighbor<T> {
    node: T,
    weight: usize,
}

impl<T> PartialEq<Self> for Neighbor<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T> Eq for Neighbor<T> {}

impl<T> PartialOrd<Self> for Neighbor<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Neighbor<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}