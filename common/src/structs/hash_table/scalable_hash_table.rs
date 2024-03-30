//! Scalable hash table implementation.

use crate::{
    base::cast::usize::Usize,
    structs::hash_table::{
        hash::{custom_hashable::CustomHash, hash},
        vectors::{
            hash_vec::{HashVec, Indexes, InternalStatistics},
            hash_vec_iterator::HashVecIterator,
            key_value::KeyValue,
            static_hash_vec::StaticHashVec,
        },
        ExtendedFunctions, HashTable, VecFunctions,
    },
};
use std::marker::PhantomData;

const MAX_BUCKET_LEN: usize = 10;

/// [`ScalableHashTable`] is a hash table with a scalable size.
/// It is using [`HashVec`] as a storage.
/// * `K` - key type
/// * `V` - value type
/// * `H` - [`HashVec`] implementation
#[derive(Debug)]
pub struct ScalableHashTable<K, V, H = StaticHashVec<K, V>> {
    /// [`HashVec`] implementation for storing key-value pairs.
    table: H,

    /// Number of elements in the hash table.
    len: usize,

    /// Hash function.
    hash: fn(&[u8]) -> u64,

    /// Value type.
    v: PhantomData<V>,

    /// Key type.
    k: PhantomData<K>,
}

impl<K, V, H> ScalableHashTable<K, V, H>
where
    H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
    K: CustomHash,
{
    /// Get max bucket length.
    /// # Returns
    /// * `usize` - Max bucket length.
    fn get_max_bucket_len(&self) -> usize {
        self.table.get_max_len()
    }

    /// Check if bucket length is greater than max bucket length.
    /// If it is, resize the bucket.
    /// # Arguments
    /// * `index` - Index of the bucket.
    fn check_bucket_len(&mut self, index: usize) {
        let bucket_len = self.table.get_bucket_len(index).unwrap();

        if bucket_len > MAX_BUCKET_LEN {
            self.resize(self.table.size() * 2);
        }
    }

    fn resize(&mut self, new_size: usize) {
        let mut new_table = H::new(new_size);
        let iter = HashVecIterator::new(&mut self.table);

        for key_value in iter {
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
    K: Clone + CustomHash,
    V: Clone,
{
    fn new(size: usize) -> Self {
        ScalableHashTable {
            table: H::new(size),
            len: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<KeyValue<K, V>> {
        let hash = key.hash(self.hash);
        let index = hash.to_usize() & (self.table.size() - 1);

        let mut result = KeyValue::new(key.clone(), value.clone());

        let has_key = self.table.have_key(index, &key);
        if has_key {
            let updated = self.table.update(index, &key, value).unwrap().value;
            result.value = updated;
        } else {
            self.table.push(index, key, value);
            self.len += 1;
        }

        self.check_bucket_len(index);

        Some(result)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
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

    fn get(&mut self, key: &K) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash.to_usize() & (self.table.size() - 1);

        let item = self.table.get(index, key);
        item.map(|item| item.value)
    }

    fn size(&self) -> usize {
        self.table.size()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K, V, H> VecFunctions<K, V> for ScalableHashTable<K, V, H>
where
    H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
    K: Copy + CustomHash,
    V: Copy,
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

        let iter = HashVecIterator::new(&mut self.table);
        for key_value in iter {
            values.push(key_value.value);
        }

        values
    }

    fn get_key_values(&mut self) -> Vec<KeyValue<K, V>> {
        let mut key_values = Vec::new();

        let iter = HashVecIterator::new(&mut self.table);
        for key_value in iter {
            key_values.push(key_value);
        }

        key_values
    }
}

impl<K, V, H> ExtendedFunctions<K, V> for ScalableHashTable<K, V, H>
where
    H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
    K: Clone + CustomHash,
    V: Clone,
{
    fn new_with_hash(size: usize, hash: fn(&[u8]) -> u64) -> Self {
        ScalableHashTable {
            table: H::new(size),
            len: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }

    fn insert_key_value(
        &mut self,
        key_value: KeyValue<K, V>,
    ) -> Option<KeyValue<K, V>> {
        self.insert(key_value.key, key_value.value)
    }

    fn insert_tuple(&mut self, tuple: (K, V)) -> Option<KeyValue<K, V>> {
        self.insert(tuple.0, tuple.1)
    }
}

impl<K, V, H> Default for ScalableHashTable<K, V, H>
where
    H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V>,
    K: Clone + CustomHash,
    V: Clone,
{
    fn default() -> Self {
        ScalableHashTable {
            table: H::new(8),
            len: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }
}

impl<K, V, H> Clone for ScalableHashTable<K, V, H>
where
    H: HashVec<K, V> + InternalStatistics<K, V> + Indexes<K, V> + Clone,
    K: Clone + CustomHash,
    V: Clone,
{
    fn clone(&self) -> Self {
        ScalableHashTable {
            table: self.table.clone(),
            len: self.len,
            v: PhantomData,
            k: PhantomData,
            hash: self.hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::hash_table::{
        scalable_hash_table::ScalableHashTable,
        vectors::{key_value::KeyValue, static_hash_vec::StaticHashVec},
        ExtendedFunctions, HashTable, VecFunctions,
    };

    #[test]
    fn test_scalable_hash_table_new() {
        let hash_table: ScalableHashTable<u64, u64, StaticHashVec<u64, u64>> =
            ScalableHashTable::new(8);

        assert_eq!(hash_table.len, 0);
        assert_eq!(hash_table.table.size, 8);
    }

    #[test]
    fn test_scalable_hash_table_insert() {
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);
    }

    #[test]
    fn test_scalable_hash_table_insert_existing() {
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        let result = hash_table.insert(0, 10);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), KeyValue::new(0, 10));

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);
        assert_eq!(hash_table.get(&0), Some(10));
    }

    #[test]
    fn test_scalable_hash_table_insert_resize() {
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

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
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        hash_table.remove(&0);

        assert_eq!(hash_table.len, 7);
        assert_eq!(hash_table.table.size, 8);
        assert_eq!(hash_table.get(&0), None);
    }

    #[test]
    fn test_scalable_hash_table_get() {
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len, 8);
        assert_eq!(hash_table.table.size, 8);

        assert_eq!(hash_table.get(&0), Some(0));
        assert_eq!(hash_table.get(&10), None);
    }

    #[test]
    fn test_scalable_hash_table_get_keys() {
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

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
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

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
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

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
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

        hash_table.insert_key_value(KeyValue::new(0, 0));

        assert_eq!(hash_table.len, 1);
        assert_eq!(hash_table.table.size, 8);
    }

    #[test]
    fn test_scalable_hash_table_insert_tuple() {
        let mut hash_table: ScalableHashTable<
            u64,
            u64,
            StaticHashVec<u64, u64>,
        > = ScalableHashTable::new(8);

        hash_table.insert_tuple((0, 0));

        assert_eq!(hash_table.len, 1);
        assert_eq!(hash_table.table.size, 8);
    }
}
