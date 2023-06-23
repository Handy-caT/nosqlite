use std::marker::PhantomData;
use crate::core::structs::hash_table::hash::custom_hash;
use crate::core::structs::hash_table::hash_table::HashTable;
use crate::core::structs::hash_table::vectors::hash_vec::HashVec;

/// StaticHashTable is a hash table with a fixed size. It is using HashVec as a storage.
/// * `K` - key type
/// * `V` - value type
/// * `N` - size of the hash table
/// * `H` - HashVec implementation
struct StaticHashTable<K, V, H, const N: u64>
    where
        H: HashVec<V, N>
{
    table: H,
    size: usize,
    v: PhantomData<V>,
    k: PhantomData<K>,
    hash: fn(&[u8]) -> u64,
}

impl <K, V, H, const N: u64> StaticHashTable<K, V, H, N>
    where
        H: HashVec<V, N>
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
            hash: custom_hash
        }
    }

    /// Creates a new StaticHashTable with a custom hash function
    /// # Arguments
    /// * `table` - HashVec implementation
    /// * `hash` - Hash function fn(&[u8]) -> u64
    /// # Returns
    /// * `Self` - StaticHashTable
    fn new_with_hash(table: H, hash: fn(&[u8]) -> u64) -> Self {
        StaticHashTable {
            table,
            size: 0,
            v: PhantomData,
            k: PhantomData,
            hash
        }
    }
}

impl <K, V, H, const N: u64> HashTable<K, V> for StaticHashTable<K, V, H, N>
    where
        H: HashVec<V, N>,
        K: Eq + Copy
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key_bytes = key.to_be_bytes();
    }

    fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    fn get(&self, key: K) -> Option<V> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}
