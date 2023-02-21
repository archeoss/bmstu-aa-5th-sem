use serde_derive::Deserialize;
use std::cmp::{Ord, Ordering, PartialOrd};

pub type Moa = f32;
pub type Name = String;

#[derive(Clone, Debug, Deserialize)]
pub struct DictEntry<K: Clone + Ord, V: PartialOrd + Copy> {
    pub key: K,
    pub value: V,
}

pub trait Map<K: Clone + Ord, V: PartialOrd + Copy> {
    fn get(&self, key: &K) -> Option<V>;
    fn search(&self, interval: (V, V)) -> Vec<DictEntry<K, V>>;
}

#[derive(Debug)]
pub struct BruteMap<K: Clone + Ord, V: PartialOrd + Copy> {
    data: Vec<DictEntry<K, V>>,
}

impl<K: Clone + Ord, V: PartialOrd + Copy> From<Vec<DictEntry<K, V>>> for BruteMap<K, V> {
    fn from(source: Vec<DictEntry<K, V>>) -> BruteMap<K, V> {
        BruteMap { data: source }
    }
}

impl<K: Clone + Ord, V: PartialOrd + Copy> Map<K, V> for BruteMap<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        self.data
            .iter()
            .find(|&e| e.key == *key)
            .map(|entry| entry.value.clone())
    }
    fn search(&self, interval: (V, V)) -> Vec<DictEntry<K, V>> {
        let mut res = Vec::new();
        for entry in &self.data {
            if entry.value >= interval.0 && entry.value < interval.1 {
                res.push(entry.clone());
            }
        }

        res
    }
}

#[cfg(test)]
mod tests;
