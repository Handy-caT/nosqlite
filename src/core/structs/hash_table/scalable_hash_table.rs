use crate::core::structs::hash_table::{ExtendedFunctions, hash::hash, HashTable, VecFunctions, vectors::hash_vec::HashVec};
use std::marker::PhantomData;
use crate::core::base::cast::usize::Usize;
use crate::core::structs::hash_table::hash::custom_hashable::CustomHash;
use crate::core::structs::hash_table::vectors::hash_vec::{Indexes, InternalStatistics};
use crate::core::structs::hash_table::vectors::hash_vec_iterator::HashVecIterator;
use crate::core::structs::hash_table::vectors::key_value::KeyValue;

const MAX_BUCKET_LEN: usize = 10;

/// [`ScalableHashTable`] is a hash table with a scalable size.
/// It is using [`HashVec`] as a storage.
/// * `K` - key type
/// * `V` - value type
/// * `H` - [`HashVec`] implementation
struct ScalableHashTable<K, V, H>
where
    H: HashVec<K, V>,
{
    table: H,
    len: usize,
    v: PhantomData<V>,
    k: PhantomData<K>,
    hash: fn(&[u8]) -> u64,
}

impl<K, V, H> ScalableHashTable<K, V, H>
where
    H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
    K: Eq + Copy + CustomHash,
    V: Eq + Copy,
{
    /// Creates a new [`ScalableHashTable`]
    /// # Arguments
    /// * `table` - [`HashVec`] implementation
    /// # Returns
    /// * `Self` - [`ScalableHashTable`]
    fn new(table: H) -> Self {
        ScalableHashTable {
            table,
            len: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }

    /// Creates a new [`ScalableHashTable`] with a custom hash function
    /// # Arguments
    /// * `table` - [`HashVec`] implementation
    /// * `hash` - Hash function fn(&[u8]) -> u64
    /// # Returns
    /// * `Self` - [`ScalableHashTable`]
    fn new_with_hash(table: H, hash: fn(&[u8]) -> u64) -> Self {
        ScalableHashTable {
            table,
            len: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }

    /// Get max bucket length.
    /// # Returns
    /// * `usize` - Max bucket length
    fn get_max_bucket_len(&self) -> usize {
        self.table.get_max_len()
    }

    /// Check if bucket length is greater than max bucket length.
    /// If it is, resize the bucket.
    /// # Arguments
    /// * `index` - Index of the bucket
    /// # Returns
    /// * `()` - Nothing
    fn check_bucket_len(&mut self, index: usize) {
        let bucket_len = self.table.get_bucket_len(index).unwrap();

        if bucket_len > MAX_BUCKET_LEN {
            self.resize(self.table.size() * 2);
        }
    }

    fn resize(&mut self, new_size: usize) {
        let mut new_table = H::new(new_size);
        let mut iter = HashVecIterator::new(&mut self.table);

        while let Some(key_value) = iter.next() {
            let key = key_value.key;
            let value = key_value.value;

            let hash = key.hash(self.hash);
            let index = hash.to_usize() & (new_size - 1);

            new_table.push(index, key, value);
        }

        self.table = new_table;
    }
}

impl<K, V, H> HashTable<K, V> for ScalableHashTable<K, V, H>
    where
        H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
        K: Eq + Copy + CustomHash,
        V: Eq + Copy,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash.to_usize() & (self.table.size() - 1);

        let has_key = self.table.have_key(index, key);
        if has_key {
            self.table.update(index, key, value);
        } else {
            self.table.push(index, key, value);
            self.len += 1;
        }

        self.check_bucket_len(index);

        Some(value)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash.to_usize() & (self.table.size() - 1);

        let item = self.table.remove(index, key);
        match item {
            Some(item) => {
                self.len -= 1;
                Some(item.value)
            }
            None => None,
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash.to_usize() & (self.table.size() - 1);

        let item = self.table.get(index, key);
        item.map(|item| item.value)
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<K, V, H> VecFunctions<K, V> for ScalableHashTable<K, V, H>
    where
        H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
        K: Eq + Copy + CustomHash,
        V: Eq + Copy,
{
    fn get_keys(&mut self) -> Vec<K> {
        let mut keys = Vec::new();

        let iter = HashVecIterator::new(&mut self.table);
        for key_value in iter {
            keys.push(key_value.key);
        }

        keys
    }

    fn get_values(&mut self) -> Vec<V> {
        let mut values = Vec::new();

        let mut iter = HashVecIterator::new(&mut self.table);
        for key_value in iter {
            values.push(key_value.value);
        }

        values
    }

    fn get_key_values(&mut self) -> Vec<KeyValue<K, V>> {
        let mut key_values = Vec::new();

        let mut iter = HashVecIterator::new(&mut self.table);
        for key_value in iter {
            key_values.push(key_value);
        }

        key_values
    }
}

impl<K, V, H> ExtendedFunctions<K, V>
for ScalableHashTable<K, V, H>
    where
        H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
        K: Eq + Copy + CustomHash,
        V: Eq + Copy,
{
    fn insert_key_value(&mut self, key_value: KeyValue<K, V>) -> Option<V> {
        self.insert(key_value.key, key_value.value)
    }

    fn insert_tuple(&mut self, tuple: (K, V)) -> Option<V> {
        self.insert(tuple.0, tuple.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::{ExtendedFunctions, HashTable, scalable_hash_table::ScalableHashTable, VecFunctions, vectors::static_hash_vec::StaticHashVec};
    use crate::core::structs::hash_table::vectors::hash_vec::HashVec;
    use crate::core::structs::hash_table::vectors::key_value::KeyValue;

    #[test]
    fn test_scalable_hash_table_new() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let hash_table = ScalableHashTable::new(hash_vec);

        assert_eq!(hash_table.len, 0);
        assert_eq!(hash_table.table.size, 8);
    }

    #[test]
    fn test_scalable_hash_table_insert() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);
    }

    #[test]
    fn test_scalable_hash_table_insert_existing() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        hash_table.insert(0, 10);

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);
        assert_eq!(hash_table.get(0), Some(10));
    }

    #[test]
    fn test_scalable_hash_table_insert_resize() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        for i in 8..80 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 80);
        assert_eq!(hash_table.table.size, 16);

    }

    #[test]
    fn test_scalable_hash_table_remove() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        hash_table.remove(0);

        assert_eq!(hash_table.len, 7);
        assert_eq!(hash_table.table.size, 8);
        assert_eq!(hash_table.get(0), None);
    }

    #[test]
    fn test_scalable_hash_table_get() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        assert_eq!(hash_table.get(0), Some(0));
        assert_eq!(hash_table.get(10), None);
    }

    #[test]
    fn test_scalable_hash_table_get_keys() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, 20 - i);
        }

        let keys = hash_table.get_keys();

        assert_eq!(keys.len(), 8);
        assert!(keys.contains(&0));
        assert!(keys.contains(&7));
    }

    #[test]
    fn test_scalable_hash_table_get_values() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, 20 - i);
        }

        let values = hash_table.get_values();

        assert_eq!(values.len(), 8);
        assert!(values.contains(&13));
        assert!(values.contains(&20));
    }

    #[test]
    fn test_scalable_hash_table_get_key_values() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table = ScalableHashTable::new(hash_vec);

        for i in 0..8 {
            hash_table.insert(i, 20 - i);
        }

        let key_values = hash_table.get_key_values();

        assert_eq!(key_values.len(), 8);
        assert!(key_values.contains(&KeyValue::new(0, 20)));
        assert!(key_values.contains(&KeyValue::new(7, 13)));
    }

    #[test]
    fn test_scalable_hash_table_insert_key_value() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table: ScalableHashTable<u64, u64, StaticHashVec<u64, u64>> = ScalableHashTable::new(hash_vec);

        hash_table.insert_key_value(KeyValue::new(0, 0));

        assert_eq!(hash_table.len, 1);
        assert_eq!(hash_table.table.size, 8);
    }

    #[test]
    fn test_scalable_hash_table_insert_tuple() {
        let hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);
        let mut hash_table: ScalableHashTable<u64, u64, StaticHashVec<u64, u64>> = ScalableHashTable::new(hash_vec);

        hash_table.insert_tuple((0, 0));

        assert_eq!(hash_table.len, 1);
        assert_eq!(hash_table.table.size, 8);
    }

}
