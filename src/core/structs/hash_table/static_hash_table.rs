use std::marker::PhantomData;
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
}

impl <K, V, H, const N: u64> StaticHashTable<K, V, H, N>
    where
        H: HashVec<V, N>
{
    /// Creates a new StaticHashTable
    fn new(table: H) -> Self {
        StaticHashTable {
            table,
            size: 0,
            v: PhantomData,
            k: PhantomData,
        }
    }
}

impl <K, V, H, const N: u64> HashTable<K, V> for StaticHashTable<K, V, H, N>
    where
        H: HashVec<V, N>
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }

    fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    fn get(&self, key: K) -> Option<&V> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}
