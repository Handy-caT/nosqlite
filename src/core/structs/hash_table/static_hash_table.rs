use crate::core::{
    base::cast::usize::UsizeCast as _,
    structs::hash_table::{
        hash::{custom_hash::CustomHash, hash::custom_hash},
        hash_table::{HashTable, HashTableExtended, HashTableVectors},
        vectors::{
            hash_vec::{HashVec, HashVecIndexes, HashVecStatisticsInternal},
            key_value::KeyValue,
        },
    },
};
use std::marker::PhantomData;

/// StaticHashTable is a hash table with a fixed size.
/// It is using HashVec as a storage.
/// * `K` - key type
/// * `V` - value type
/// * `N` - size of the hash table
/// * `H` - HashVec implementation
struct StaticHashTable<K, V, H, const N: usize>
where
    H: HashVec<K, V>,
{
    table: H,
    size: usize,
    v: PhantomData<V>,
    k: PhantomData<K>,
    hash: fn(&[u8]) -> u64,
}

impl<K, V, H, const N: usize> StaticHashTable<K, V, H, N>
where
    H: HashVec<K, V>,
{
    /// Creates a new StaticHashTable
    /// # Arguments
    /// * `table` - HashVec implementation
    /// # Returns
    /// * `Self` - StaticHashTable
    fn new(table: H) -> Self {
        StaticHashTable {
            table,
            size: 0,
            v: PhantomData,
            k: PhantomData,
            hash: custom_hash,
        }
    }

    /// Creates a new StaticHashTable with a custom hash function
    /// # Arguments
    /// * `table` - HashVec implementation
    /// * `hash` - Hash function fn(&[u8]) -> usize
    /// # Returns
    /// * `Self` - StaticHashTable
    fn new_with_hash(table: H, hash: fn(&[u8]) -> u64) -> Self {
        StaticHashTable {
            table,
            size: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }
}

impl<K, V, H, const N: usize> HashTable<K, V> for StaticHashTable<K, V, H, N>
where
    H: HashVec<K, V>,
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
            self.size += 1;
        }

        Some(value)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash.to_usize() & (self.table.size() - 1);

        let item = self.table.remove(index, key);
        match item {
            Some(item) => {
                self.size -= 1;
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
        self.size
    }
}

impl<K, V, H, const N: usize> HashTableVectors<K, V>
    for StaticHashTable<K, V, H, N>
where
    H: HashVec<K, V> + HashVecStatisticsInternal<K, V> + HashVecIndexes<K, V>,
    K: Eq + Copy + CustomHash,
    V: Eq + Copy,
{
    fn get_keys(&mut self) -> Vec<K> {
        let mut keys = Vec::<K>::new();

        for i in 0..self.table.size() {
            let len = self.table.get_bucket_len(i).unwrap();
            for j in 0..len {
                keys.push(self.table.get_by_index(i, j).unwrap().key);
            }
        }

        keys
    }

    fn get_values(&mut self) -> Vec<V> {
        let mut values = Vec::<V>::new();

        for i in 0..self.table.size() {
            let len = self.table.get_bucket_len(i).unwrap();
            for j in 0..len {
                values.push(self.table.get_by_index(i, j).unwrap().value);
            }
        }

        values
    }

    fn get_key_values(&mut self) -> Vec<KeyValue<K, V>> {
        let mut values = Vec::<KeyValue<K, V>>::new();

        for i in 0..self.table.size() {
            let len = self.table.get_bucket_len(i).unwrap();
            for j in 0..len {
                values.push(self.table.get_by_index(i, j).unwrap());
            }
        }

        values
    }
}

impl<K, V, H, const N: usize> HashTableExtended<K, V>
    for StaticHashTable<K, V, H, N>
where
    H: HashVec<K, V>,
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
    use crate::core::structs::hash_table::{
        hash_table::{HashTable, HashTableExtended, HashTableVectors},
        static_hash_table::StaticHashTable,
        vectors::{key_value::KeyValue, static_hash_vec::StaticHashVec},
    };

    #[test]
    fn test_static_hash_table_new() {
        let hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        assert_eq!(hash_table.len(), 0);
    }

    #[test]
    fn test_static_hash_table_insert() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len(), 8);
    }

    #[test]
    fn test_static_hash_table_insert_existing() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len(), 8);

        hash_table.insert(0, 10);

        assert_eq!(hash_table.len(), 8);
        assert_eq!(hash_table.get(0), Some(10));
    }

    #[test]
    fn test_static_hash_table_remove() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        let item = hash_table.remove(0);
        assert_eq!(item, Some(0));
        assert_eq!(hash_table.len(), 7);

        for i in 1..8 {
            hash_table.remove(i);
        }

        assert_eq!(hash_table.len(), 0);

        let item = hash_table.remove(0);
        assert_eq!(item, None);
    }

    #[test]
    fn test_static_hash_table_get() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        let item = hash_table.get(0);
        assert_eq!(item, Some(0));

        let item = hash_table.get(8);
        assert_eq!(item, None);
    }

    #[test]
    fn test_static_hash_table_get_keys() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        let keys = hash_table.get_keys();
        assert_eq!(keys.len(), 8);
    }

    #[test]
    fn test_static_hash_table_get_values() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        let values = hash_table.get_values();
        assert_eq!(values.len(), 8);
    }

    #[test]
    fn test_static_hash_table_get_key_values() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        let key_values = hash_table.get_key_values();
        assert_eq!(key_values.len(), 8);
    }

    #[test]
    fn test_static_hash_table_insert_key_value() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());
        let key_value = hash_table.insert_key_value(KeyValue::new(0, 0));
        assert_eq!(key_value, Some(0));

        let key_value = hash_table.insert_key_value(KeyValue::new(1, 1));
        assert_eq!(key_value, Some(1));
    }

    #[test]
    fn test_static_hash_table_insert_tuple() {
        let mut hash_table: StaticHashTable<
            usize,
            usize,
            StaticHashVec<usize, usize, 8>,
            8,
        > = StaticHashTable::new(StaticHashVec::new());
        let tuple = hash_table.insert_tuple((0, 0));
        assert_eq!(tuple, Some(0));

        let tuple = hash_table.insert_tuple((1, 1));
        assert_eq!(tuple, Some(1));
    }
}
