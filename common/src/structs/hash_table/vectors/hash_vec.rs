use crate::structs::hash_table::vectors::{
    key_value::KeyValue, statistics::hash_vec,
};

/// Common trait for [`HashTable`] data.
pub trait HashVec<K, V> {
    /// Creates a new [`HashVec`].
    /// # Arguments
    /// * `size` - Size of [`HashVec`].
    /// # Returns
    /// * `StaticHashVec<V, N>` - New [`HashVec`].
    fn new(size: usize) -> Self;

    /// Adds value to the [`HashVec`] by underlying vector index
    /// and value itself.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `key` - Key to add.
    /// * `value` - Value to add.
    /// # Returns
    /// * `(usize, usize)` - Index of the underlying vector and
    /// index of the value in the vector.
    fn push(&mut self, index: usize, key: K, value: V) -> (usize, usize);

    /// Gets value from the [`HashVec`] by underlying vector index and key.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `key` - Key to get.
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was found,
    /// None if value was not found.
    fn get(&self, index: usize, key: &K) -> Option<KeyValue<K, V>>;

    /// Updates value in the [`HashVec`] by underlying vector index and key.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `key` - Key to update.
    /// * `value` - Value to update.
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was updated,
    /// None if value was not found.
    fn update(
        &mut self,
        index: usize,
        key: &K,
        value: V,
    ) -> Option<KeyValue<K, V>>;

    /// Checks if the [`HashVec`] has key by underlying vector index.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `key` - Key to check.
    /// # Returns
    /// * `bool` - True if the [`HashVec`] has key, false otherwise.
    fn have_key(&mut self, index: usize, key: &K) -> bool;

    /// Removes value from the [`HashVec`] by underlying vector
    /// index and value itself.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `value` - Value to remove.
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was removed,
    /// None if value was not found.
    fn remove(&mut self, index: usize, key: &K) -> Option<KeyValue<K, V>>;

    /// Returns number of buckets in the [`HashVec`].
    /// # Returns
    /// * `usize` - Number of buckets in the [`HashVec`].
    fn size(&self) -> usize;

    /// Returns length of the [`HashVec`] as sum of lengths
    /// of underlying vectors.
    /// # Returns
    /// * `usize` - Length of the [`HashVec`].
    fn len(&self) -> usize;

    /// Returns true if [`HashVec`] is empty.
    /// # Returns
    /// * `bool` - State of emptiness.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Common trait for [`HashTable`] to get values by underlying vector index.
pub trait Indexes<K, V> {
    /// Removes value from the [`HashVec`] by underlying
    /// vector index and value index.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `value_index` - Index of the value in the vector.
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was removed,
    /// None if value index was out of bounds.
    fn remove_by_index(
        &mut self,
        index: usize,
        value_index: usize,
    ) -> Option<KeyValue<K, V>>;

    /// Returns reference to the value from the [`HashVec`] by underlying
    /// vector index and value index.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `value_index` - Index of the value in the vector.
    /// # Returns
    /// * `Option<KeyValue<K, V>>` - Value that was found, None if value index
    /// was out of bounds.
    fn get_by_index(
        &mut self,
        index: usize,
        value_index: usize,
    ) -> Option<KeyValue<K, V>>;

    /// Finds key in the [`HashVec`] by underlying vector index and key itself.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `key` - Key to find.
    /// # Returns
    /// * `Option<usize>` - Index of the key in the vector if it was found,
    /// None otherwise.
    fn find_key(&mut self, index: usize, key: &K) -> Option<usize>;
}

/// Internal trait for [`HashTable`] to get underlying vectors by
/// underlying vector index.
pub trait InternalFunctions<K, V> {
    /// Returns reference to the underlying vector from the [`HashVec`] by
    /// underlying vector index.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// # Returns
    /// * 'Option<&Vec<V>>' - Reference to the underlying vector,
    /// None if index was out of bounds.
    fn get_vec(&self, index: usize) -> Option<&Vec<KeyValue<K, V>>>;

    /// Returns mutable reference to the underlying vector from the [`HashVec`]
    /// by underlying vector index.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// # Returns
    /// * 'Option<&mut Vec<V>>' - Mutable reference to the underlying vector,
    /// None if index was out of bounds.
    fn get_vec_mut(&mut self, index: usize)
        -> Option<&mut Vec<KeyValue<K, V>>>;
}

/// Common trait for [`HashTable`] to get statistics about underlying vectors.
pub trait InternalStatistics<K, V> {
    /// Returns max length of the underlying vectors.
    /// # Returns
    /// * `usize` - Max length of the underlying vectors.
    fn get_max_len(&self) -> usize;

    /// Returns [`Statistics`] about underlying vectors.
    /// # Returns
    /// * `&HashVecStatistics` - Reference to the [`Statistics`] about
    /// underlying vectors.
    ///
    /// [`Statistics`]: hash_vec::Stats
    fn get_statistics(&self) -> &hash_vec::Stats;

    /// Returns mutable reference to the [`Statistics`] about underlying
    /// vectors.
    /// # Returns
    /// * `&mut HashVecStatistics` - Mutable reference to the [`Statistics`]
    /// about underlying vectors.
    ///
    /// [`Statistics`]: hash_vec::Stats
    fn get_statistics_mut(&mut self) -> &mut hash_vec::Stats;

    /// Returns length of the underlying vector by underlying vector index.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// # Returns
    /// * `Option<usize>` - Length of the underlying vector,
    /// None if index was out of bounds.
    fn get_bucket_len(&self, index: usize) -> Option<usize>;
}

/// Common trait for [`HashTable`] to get mutable reference to the value.
pub trait MutableHashVec<K, V> {
    /// Returns mutable reference to the value from the [`HashVec`] by
    /// underlying vector index and key.
    /// # Arguments
    /// * `index` - Index of the underlying vector.
    /// * `key` - Key to get.
    /// # Returns
    /// * `Option<&mut V>` - Mutable reference to the value.
    fn get_mut_value(&mut self, index: usize, key: &K) -> Option<&mut V>;
}
