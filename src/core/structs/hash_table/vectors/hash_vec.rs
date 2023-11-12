use crate::core::structs::hash_table::vectors::{
    key_value::KeyValue, statistics::hash_vec_statistics::HashVecStatistics,
};

/// Common trait for HashTable data
pub trait HashVec<K, V> {
    /// Adds value to the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `key` - Key to add
    /// * `value` - Value to add
    /// # Returns
    /// * `(u64, usize)` - Index of the underlying vector and index of the value in the vector
    fn push(&mut self, index: u64, key: K, value: V) -> (u64, usize);

    /// Gets value from the HashVector by underlying vector index and key
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `key` - Key to get
    /// # Returns
    /// * 'Option<KeyValue<K, V>>' - Value that was found, None if value was not found
    fn get(&mut self, index: u64, key: K) -> Option<KeyValue<K, V>>;

    /// Updates value in the HashVector by underlying vector index and key
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `key` - Key to update
    /// * `value` - Value to update
    /// # Returns
    /// * 'Option<KeyValue<K, V>>' - Value that was updated, None if value was not found
    fn update(
        &mut self,
        index: u64,
        key: K,
        value: V,
    ) -> Option<KeyValue<K, V>>;

    /// Checks if the HashVector has key by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `key` - Key to check
    /// # Returns
    /// * `bool` - True if the HashVector has key, false otherwise
    fn have_key(&mut self, index: u64, key: K) -> bool;

    /// Removes value from the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to remove
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was removed, None if value was not found
    fn remove(&mut self, index: u64, key: K) -> Option<KeyValue<K, V>>;

    /// Returns number of buckets in the HashVector
    /// # Returns
    /// * `u64` - Number of buckets in the HashVector
    fn size(&self) -> u64;

    /// Returns length of the HashVector as sum of lengths of underlying vectors
    /// # Returns
    /// * `u64` - Length of the HashVector
    fn len(&self) -> u64;
}

/// Common trait for HashTable to get values by underlying vector index
pub trait HashVecIndexes<K, V> {
    /// Removes value from the HashVector by underlying vector index and value index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value_index` - Index of the value in the vector
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was removed, None if value index was out of bounds
    fn remove_by_index(
        &mut self,
        index: u64,
        value_index: usize,
    ) -> Option<KeyValue<K, V>>;

    /// Returns reference to the value from the HashVector by underlying vector index and value index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value_index` - Index of the value in the vector
    /// # Returns
    /// * 'Option<KeyValue<K, V>>' - Value that was found, None if value index was out of bounds
    fn get_by_index(
        &mut self,
        index: u64,
        value_index: usize,
    ) -> Option<KeyValue<K, V>>;

    /// Finds key in the HashVector by underlying vector index and key itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `key` - Key to find
    /// # Returns
    /// * `Option<usize>` - Index of the key in the vector if it was found, None otherwise
    fn find_key(&mut self, index: u64, key: K) -> Option<usize>;
}

/// Internal trait for HashTable to get underlying vectors by underlying vector index
pub(in crate::core::structs::hash_table) trait HashVecInternal<K, V> {
    /// Returns reference to the underlying vector from the HashVector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * 'Option<&Vec<V>>' - Reference to the underlying vector, None if index was out of bounds
    fn get_vec(&self, index: u64) -> Option<&Vec<KeyValue<K, V>>>;

    /// Returns mutable reference to the underlying vector from the HashVector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * 'Option<&mut Vec<V>>' - Mutable reference to the underlying vector, None if index was out of bounds
    fn get_vec_mut(&mut self, index: u64) -> Option<&mut Vec<KeyValue<K, V>>>;
}

/// Common trait for HashTable to get statistics about underlying vectors
pub(in crate::core::structs::hash_table) trait HashVecStatisticsInternal<K, V> {
    /// Returns max length of the underlying vectors
    /// # Returns
    /// * `usize` - Max length of the underlying vectors
    fn get_max_len(&self) -> usize;

    /// Returns Statistics about underlying vectors
    /// # Returns
    /// * `&HashVecStatistics` - Reference to the Statistics about underlying vectors
    fn get_statistics(&self) -> &HashVecStatistics;

    /// Returns mutable reference to the Statistics about underlying vectors
    /// # Returns
    /// * `&mut HashVecStatistics` - Mutable reference to the Statistics about underlying vectors
    fn get_statistics_mut(&mut self) -> &mut HashVecStatistics;

    /// Returns length of the underlying vector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * `Option<usize>` - Length of the underlying vector, None if index was out of bounds
    fn get_bucket_len(&self, index: u64) -> Option<usize>;
}
