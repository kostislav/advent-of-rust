use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

use ahash::{HashMap, HashMapExt};
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


pub fn shortest_path<T, I, S, F, TF>(starting_node: T, target_node_predicate: TF, mut dist: S, edge_supplier: F) -> usize
    where T: Eq + Copy,
          I: Iterator<Item=(T, usize)>,
          S: SimpleMap<T>,
          F: Fn(T) -> I,
          TF: Fn(&T) -> bool,
{
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Neighbor::new(starting_node, 0));
    dist.insert(starting_node, 0);

    loop {
        let closest = to_visit.pop().unwrap();
        if target_node_predicate(&closest.node) {
            return closest.weight;
        } else if dist.get(&closest.node).unwrap() == closest.weight {
            for (neighbor, weight) in edge_supplier(closest.node) {
                let neighbor_weight = closest.weight + weight;
                if dist.get(&neighbor).map(|it| it > neighbor_weight).unwrap_or(true) {
                    to_visit.push(Neighbor::new(neighbor, neighbor_weight));
                    dist.insert(neighbor, neighbor_weight);
                }
            }
        }
    }
}

pub trait SimpleMap<K> {
    fn insert(&mut self, key: K, value: usize);
    fn get(&self, key: &K) -> Option<usize>;
}

impl<K: Eq + Hash> SimpleMap<K> for HashMap<K, usize> {
    fn insert(&mut self, key: K, value: usize) {
        self.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<usize> {
        self.get(key).copied()
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