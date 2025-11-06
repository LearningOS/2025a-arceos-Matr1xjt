#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
pub struct HashNode<K, V> {
    key: K,
    value: V,
}


pub fn hash<K: core::hash::Hash>(key: &K) -> usize {
    use core::hash::{Hasher, BuildHasherDefault};
    use core::hash::Hash;
    use core::hash::SipHasher;

    let mut hasher = SipHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize
}

#[cfg(feature = "alloc")]
pub struct HashMap<K, V> {
   hash : Vec<Vec<HashNode<K, V>>>,
}
#[cfg(feature = "alloc")]
impl<K: core::hash::Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        // 分配长度为13的HashNode数组
        let mut hash= Vec::with_capacity(13);
        for _ in 0..13 {
            hash.push(Vec::new());
        }
        Self {
            hash,
        }
        
    }

    pub fn insert(&mut self, key: K, value: V) {
        let node = HashNode {
            key,
            value,
        };
        let index = hash(&node.key) % self.hash.capacity();
        self.hash[index].push(node);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.hash.iter().flat_map(|bucket| {
            bucket.iter().map(|node| (&node.key, &node.value))
        })
    }
}

