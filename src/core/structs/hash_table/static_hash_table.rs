use std::marker::PhantomData;
use crate::core::structs::hash_table::hash::custom_hash::CustomHash;
use crate::core::structs::hash_table::hash::hash::custom_hash;
use crate::core::structs::hash_table::hash_table::HashTable;
use crate::core::structs::hash_table::vectors::hash_vec::HashVec;

/// StaticHashTable is a hash table with a fixed size. It is using HashVec as a storage.
/// * `K` - key type
/// * `V` - value type
/// * `N` - size of the hash table
/// * `H` - HashVec implementation
struct StaticHashTable<K, V, H, const N: u64>
    where
        H: HashVec<K, V, N>
{
    table: H,
    size: usize,
    v: PhantomData<V>,
    k: PhantomData<K>,
    hash: fn(&[u8]) -> u64,
}

impl <K, V, H, const N: u64> StaticHashTable<K, V, H, N>
    where
        H: HashVec<K, V, N>
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
        H: HashVec<K, V, N>,
        K: Eq + Copy + CustomHash,
        V: Eq + Copy
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = key.hash(self.hash);

        let index = hash & (self.table.size() - 1);
        self.table.push(index, key, value);
        self.size += 1;

        Some(value)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash & (self.table.size() - 1);

        let item = self.table.remove(index, key);
        match item {
            Some(item) => {
                self.size -= 1;
                Some(item.value)
            },
            None => None
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        let hash = key.hash(self.hash);
        let index = hash & (self.table.size() - 1);

        let item = self.table.get(index, key);
        match item {
            Some(item) => Some(item.value),
            None => None
        }
    }

    fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::hash_table::HashTable;
    use crate::core::structs::hash_table::vectors::static_hash_vec::StaticHashVec;
    use crate::core::structs::hash_table::static_hash_table::StaticHashTable;

    #[test]
    fn test_static_hash_table_new() {
        let hash_table: StaticHashTable<u64, u64, StaticHashVec<u64, u64, 8>, 8> = StaticHashTable::new(StaticHashVec::new());

        assert_eq!(hash_table.len(), 0);
    }

    #[test]
    fn test_static_hash_table_insert() {
        let mut hash_table: StaticHashTable<u64, u64, StaticHashVec<u64, u64,  8>, 8> = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        assert_eq!(hash_table.len(), 8);
    }

    #[test]
    fn test_static_hash_table_remove() {
        let mut hash_table: StaticHashTable<u64, u64, StaticHashVec<u64, u64,  8>, 8> = StaticHashTable::new(StaticHashVec::new());

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
        let mut hash_table: StaticHashTable<u64, u64, StaticHashVec<u64, u64,  8>, 8> = StaticHashTable::new(StaticHashVec::new());

        for i in 0..8 {
            hash_table.insert(i, i);
        }

        let item = hash_table.get(0);
        assert_eq!(item, Some(0));

        let item = hash_table.get(8);
        assert_eq!(item, None);
    }
}