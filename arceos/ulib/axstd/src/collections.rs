
use crate::vec::Vec;
pub struct HashMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().zip(self.values.iter())
    }
}

