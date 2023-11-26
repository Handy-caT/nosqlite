mod backwards_hash_table;
pub mod hash;
mod scalable_hash_table;
mod static_hash_table;
pub mod vectors;

use crate::core::structs::hash_table::vectors::{
    hash_vec::HashVec, key_value::KeyValue,
};

/// Trait for hash table
pub trait HashTable<K, V> {
    /// Creates a new [`HashTable`].
    /// # Arguments
    /// * `size` - Number of buckets in the [`HashTable`].
    /// # Returns
    /// * `Self` - [`HashTable`].
    fn new(size: usize) -> Self;

    /// Inserts a new value into the hash table
    /// # Arguments
    /// * `key` - key of the value
    /// * `value` - value to insert
    /// # Returns
    /// * `Option<V>` - The value inserted
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Removes a value from the hash table
    /// # Arguments
    /// * `key` - key of the value
    /// # Returns
    /// * `Option<V>` - The value removed.
    /// None if the key is not in the hash table.
    fn remove(&mut self, key: K) -> Option<V>;

    /// Gets a value from the hash table
    /// # Arguments
    /// * `key` - key of the value
    /// # Returns
    /// * `Option<V>` - The value. None if the key is not in the hash table.
    fn get(&mut self, key: K) -> Option<V>;

    /// Returns number of buckets in the [`HashTable`].
    /// # Returns
    /// * `usize` - Number of buckets in the [`HashTable`].
    fn size(&self) -> usize;

    /// Returns the number of elements in the hash table
    /// # Returns
    /// * `usize` - Number of elements in the hash table
    fn len(&self) -> usize;
}

/// Some additional methods for [`HashTable`]
pub trait ExtendedFunctions<K, V> {
    /// Creates a new [`HashTable`] with a custom hash function.
    /// # Arguments
    /// * `table` - [`HashVec`] implementation.
    /// * `hash` - Hash function fn(&u8) -> u64.
    /// # Returns
    /// * `Self` - [`HashTable`].
    fn new_with_hash(size: usize, hash: fn(&[u8]) -> u64) -> Self;

    /// Pushes key-value pair into the [`HashTable`]
    /// # Arguments
    /// * `key_value` - Key-value pair to push
    /// # Returns
    /// * `Option<V>` - Value that was inserted
    fn insert_key_value(&mut self, key_value: KeyValue<K, V>) -> Option<V>;

    /// Pushes tuple into the [`HashTable`]
    /// # Arguments
    /// * `tuple` - Tuple to push (key, value)
    /// # Returns
    /// * `Option<V>` - Value that was inserted
    fn insert_tuple(&mut self, tuple: (K, V)) -> Option<V>;
}

/// Trait for [`HashTable`] to get keys, values and key-value pairs
pub trait VecFunctions<K, V> {
    /// Returns vector of keys in the [`HashTable`]
    /// # Returns
    /// * `Vec<K>` - Vector of keys in the [`HashTable`]
    fn get_keys(&mut self) -> Vec<K>;

    /// Returns vector of values in the [`HashTable`]
    /// # Returns
    /// * `Vec<V>` - Vector of values in the [`HashTable`]
    fn get_values(&mut self) -> Vec<V>;

    /// Returns vector of key-value pairs in the [`HashTable`]
    /// # Returns
    /// * `Vec<KeyValue<K, V>>` - Vector of key-value pairs in the [`HashTable`]
    fn get_key_values(&mut self) -> Vec<KeyValue<K, V>>;
}
