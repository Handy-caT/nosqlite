
/// Trait for hash table
pub trait HashTable<K, V> {
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
    /// * `Option<V>` - The value removed. None if the key is not in the hash table.
    fn remove(&mut self, key: K) -> Option<V>;
    
    /// Gets a value from the hash table
    /// # Arguments
    /// * `key` - key of the value
    /// # Returns
    /// * `Option<V>` - The value. None if the key is not in the hash table.
    fn get(&self, key: K) -> Option<V>;

    /// Returns the number of elements in the hash table
    /// # Returns
    /// * `usize` - Number of elements in the hash table
    fn len(&self) -> usize;
}