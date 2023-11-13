use crate::core::structs::hash_table::{
    hash::hash::custom_hash, vectors::hash_vec::HashVec,
};
use std::marker::PhantomData;

/// ScalableHashTable is a hash table with a scalable size.
/// It is using HashVec as a storage.
/// * `K` - key type
/// * `V` - value type
/// * `H` - HashVec implementation
struct ScalableHashTable<K, V, H>
where
    H: HashVec<K, V>,
{
    table: H,
    size: usize,
    v: PhantomData<V>,
    k: PhantomData<K>,
    hash: fn(&[u8]) -> u64,
}

impl<K, V, H> ScalableHashTable<K, V, H>
where
    H: HashVec<K, V>,
{
    /// Creates a new ScalableHashTable
    /// # Arguments
    /// * `table` - HashVec implementation
    /// # Returns
    /// * `Self` - ScalableHashTable
    fn new(table: H) -> Self {
        ScalableHashTable {
            table,
            size: 0,
            v: PhantomData,
            k: PhantomData,
            hash: custom_hash,
        }
    }

    /// Creates a new ScalableHashTable with a custom hash function
    /// # Arguments
    /// * `table` - HashVec implementation
    /// * `hash` - Hash function fn(&[u8]) -> u64
    /// # Returns
    /// * `Self` - ScalableHashTable
    fn new_with_hash(table: H, hash: fn(&[u8]) -> u64) -> Self {
        ScalableHashTable {
            table,
            size: 0,
            v: PhantomData,
            k: PhantomData,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::{
        scalable_hash_table::ScalableHashTable,
        vectors::static_hash_vec::StaticHashVec,
    };

    #[test]
    fn test_scalable_hash_table_new() {
        let hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();
        let hash_table = ScalableHashTable::new(hash_vec);

        assert_eq!(hash_table.size, 0);
        assert_eq!(hash_table.table.size, 8);
    }
}
