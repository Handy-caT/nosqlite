use crate::core::{
    base::vector::optimized_vector::OptimizedVector,
    structs::hash_table::{
        hash::custom_hashable::CustomHash, vectors::key_value::KeyValue,
        ExtendedFunctions, HashTable,
    },
};
use crate::core::structs::hash_table::VecFunctions;

pub struct BackwardsHashTable<HK, HV, K, V> {
    /// Vector of key-value pairs.
    key_values: OptimizedVector<KeyValue<K, V>>,

    /// [`HashTable`] for storing key-value pairs hashed by key.
    key_hash_table: HK,

    /// [`HashTable`] for storing key-value pairs hashed by value.
    value_hash_table: HV,

    /// Number of elements in the hash table.
    len: usize,
}

impl<HK, HV, K, V> BackwardsHashTable<HK, HV, K, V>
where
    HK: HashTable<K, usize>,
    HV: HashTable<V, usize>,
    K: Copy + CustomHash,
    V: Copy,
{
    /// Removes a key-value pair by value.
    /// # Arguments
    /// * `value` - Value of the key-value pair to remove.
    /// # Returns
    /// * `Option<K>` - Key of the key-value pair that was removed.
    pub fn remove_by_value(&mut self, value: V) -> Option<K> {
        let value_index = self.value_hash_table.remove(value)?;
        let key_value = self.key_values.remove(value_index)?;

        self.key_hash_table.remove(key_value.key)?;

        self.len -= 1;

        Some(key_value.key)
    }

    /// Gets a key by value.
    /// # Arguments
    /// * `value` - Value of the key-value pair to get.
    /// # Returns
    /// * `Option<K>` - Key of the key-value pair that was removed.
    pub fn get_by_value(&mut self, value: V) -> Option<K> {
        let value_index = self.value_hash_table.get(value)?;
        let key_value = self.key_values.get(value_index)?;

        Some(key_value.key)
    }
}

impl<HK, HV, K, V> HashTable<K, V> for BackwardsHashTable<HK, HV, K, V>
where
    HK: HashTable<K, usize>,
    HV: HashTable<V, usize>,
    K: Copy + CustomHash,
    V: Copy + CustomHash,
{
    fn new(size: usize) -> Self {
        let key_hash_table = HK::new(size);
        let value_hash_table = HV::new(size);
        let key_values = OptimizedVector::<KeyValue<K, V>>::new();

        BackwardsHashTable {
            key_values,
            key_hash_table,
            value_hash_table,
            len: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.key_values.push(KeyValue::new(key, value));

        self.key_hash_table.insert(key, index);
        self.value_hash_table.insert(value, index);

        self.len += 1;

        Some(value)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let key_index = self.key_hash_table.remove(key)?;
        let key_value = self.key_values.remove(key_index)?;

        self.value_hash_table.remove(key_value.value)?;

        self.len -= 1;

        Some(key_value.value)
    }

    fn get(&mut self, key: K) -> Option<V> {
        let key_index = self.key_hash_table.get(key)?;
        let key_value = self.key_values.get(key_index)?;

        Some(key_value.value)
    }

    fn size(&self) -> usize {
        self.key_hash_table.size()
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<HK, HV, K, V> ExtendedFunctions<K, V> for BackwardsHashTable<HK, HV, K, V>
where
    HK: HashTable<K, usize> + ExtendedFunctions<K, usize>,
    HV: HashTable<V, usize> + ExtendedFunctions<V, usize>,
    K: Copy + CustomHash,
    V: Copy + CustomHash,
{
    fn new_with_hash(size: usize, hash: fn(&[u8]) -> u64) -> Self {
        let key_hash_table = HK::new_with_hash(size, hash);
        let value_hash_table = HV::new_with_hash(size, hash);
        let key_values = OptimizedVector::<KeyValue<K, V>>::new();

        BackwardsHashTable {
            key_values,
            key_hash_table,
            value_hash_table,
            len: 0,
        }
    }

    fn insert_key_value(&mut self, key_value: KeyValue<K, V>) -> Option<V> {
        self.insert(key_value.key, key_value.value)
    }

    fn insert_tuple(&mut self, tuple: (K, V)) -> Option<V> {
        self.insert(tuple.0, tuple.1)
    }
}

impl<HK, HV, K, V> VecFunctions<K, V> for BackwardsHashTable<HK, HV, K, V>
    where
        HK: HashTable<K, usize> + VecFunctions<K, usize>,
        HV: HashTable<V, usize> + VecFunctions<V, usize>,
        K: Copy + CustomHash,
        V: Copy + CustomHash,
{
    fn get_keys(&mut self) -> Vec<K> {
        self.key_hash_table.get_keys()
    }

    fn get_values(&mut self) -> Vec<V> {
        self.value_hash_table.get_keys()
    }

    fn get_key_values(&mut self) -> Vec<KeyValue<K, V>> {
        self.key_values.get_data().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::{backwards_hash_table::BackwardsHashTable, static_hash_table::StaticHashTable, vectors::key_value::KeyValue, ExtendedFunctions, HashTable, VecFunctions};

    #[test]
    fn test_backwards_hash_table_new() {
        let hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        assert_eq!(hash_table.key_values.len(), 0);
        assert_eq!(hash_table.key_hash_table.size(), 16);

        assert_eq!(hash_table.size(), 16);

        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 0);

        assert_eq!(hash_table.len(), 0);
    }

    #[test]
    fn test_backwards_hash_table_insert() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        let value = hash_table.insert(1, 2);

        assert_eq!(value, Some(2));
        assert_eq!(hash_table.key_values.len(), 1);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 1);
    }

    #[test]
    fn test_backwards_hash_table_remove() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        hash_table.insert(1, 2);
        let value = hash_table.remove(1);

        assert_eq!(value, Some(2));
        assert_eq!(hash_table.key_values.len(), 0);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 0);
    }

    #[test]
    fn test_backwards_hash_table_remove_by_value() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        hash_table.insert(1, 2);
        let key = hash_table.remove_by_value(2);

        assert_eq!(key, Some(1));
        assert_eq!(hash_table.key_values.len(), 0);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 0);
    }

    #[test]
    fn test_backwards_hash_table_get() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        hash_table.insert(1, 2);
        let value = hash_table.get(1);

        assert_eq!(value, Some(2));
        assert_eq!(hash_table.key_values.len(), 1);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 1);
    }

    #[test]
    fn test_backwards_hash_table_get_by_value() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        hash_table.insert(1, 2);
        let key = hash_table.get_by_value(2);

        assert_eq!(key, Some(1));
        assert_eq!(hash_table.key_values.len(), 1);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 1);
    }

    #[test]
    fn test_backwards_hash_table_insert_key_value() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        let value = hash_table.insert_key_value(KeyValue::new(1, 2));

        assert_eq!(value, Some(2));
        assert_eq!(hash_table.key_values.len(), 1);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 1);
    }

    #[test]
    fn test_backwards_hash_table_insert_tuple() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(10);

        let value = hash_table.insert_tuple((1, 2));

        assert_eq!(value, Some(2));
        assert_eq!(hash_table.key_values.len(), 1);
        assert_eq!(hash_table.key_hash_table.size(), 16);
        assert_eq!(hash_table.value_hash_table.size(), 16);
        assert_eq!(hash_table.len, 1);
    }

    #[test]
    fn test_backwards_hash_table_get_keys() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(8);

        for i in 0..8 {
            hash_table.insert(i, 20 - i);
        }

        let keys = hash_table.get_keys();
        assert_eq!(keys.len(), 8);
        assert!(keys.contains(&0));
        assert!(keys.contains(&7));
    }

    #[test]
    fn test_backwards_hash_table_get_values() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(8);

        for i in 0..8 {
            hash_table.insert(i, 20 - i);
        }

        let values = hash_table.get_values();
        assert_eq!(values.len(), 8);
        assert!(values.contains(&13));
        assert!(values.contains(&20));
    }

    #[test]
    fn test_backwards_hash_table_get_key_values() {
        let mut hash_table = BackwardsHashTable::<
            StaticHashTable<usize, usize>,
            StaticHashTable<usize, usize>,
            usize,
            usize,
        >::new(8);

        for i in 0..8 {
            hash_table.insert(i, 20 - i);
        }

        let key_values = hash_table.get_key_values();
        assert_eq!(key_values.len(), 8);
        assert!(key_values.contains(&KeyValue::new(0, 20)));
        assert!(key_values.contains(&KeyValue::new(7, 13)));
    }
}
