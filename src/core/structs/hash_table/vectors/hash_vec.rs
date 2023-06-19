use crate::core::structs::hash_table::vectors::statistics::hash_vec_statistics::HashVecStatistics;

/// Common trait for HashTable data
pub trait HashVec<V, const N: u64> {
    /// Adds value to the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to add
    /// # Returns
    /// * `(u64, usize)` - Index of the underlying vector and index of the value in the vector
    fn push(&mut self, index: u64, value: V) -> (u64, usize);

    /// Checks if the HashVector has value by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to check
    /// # Returns
    /// * `(bool, usize)` - True if the HashVector has value, false otherwise and index of the value in the vector
    fn have_item(&mut self, index: u64, value: V) -> bool;

    /// Finds value in the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to find
    /// # Returns
    /// * `Option<usize>` - Index of the value in the vector if it was found, None otherwise
    fn find_item(&mut self, index: u64, value: V) -> Option<usize>;

    /// Removes value from the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to remove
    /// # Returns
    /// * `Option<V>` - Value that was removed, None if value was not found
    fn remove(&mut self, index: u64, value: V) -> Option<V>;

    /// Returns length of the HashVector as sum of lengths of underlying vectors
    /// # Returns
    /// * `u64` - Length of the HashVector
    fn len(&self) -> u64;
}

/// Common trait for HashTable to get values by underlying vector index
pub trait HashVecIndexes<V, const N: u64> {
    /// Removes value from the HashVector by underlying vector index and value index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value_index` - Index of the value in the vector
    /// # Returns
    /// * `Option<V>` - Value that was removed, None if value index was out of bounds
    fn remove_by_index(&mut self, index: u64, value_index: usize) -> Option<V>;

    /// Returns reference to the value from the HashVector by underlying vector index and value index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value_index` - Index of the value in the vector
    /// # Returns
    /// * 'Option<&V>' - Reference to the value, None if value index was out of bounds
    fn get_by_index(&mut self, index: u64, value_index: usize) -> Option<V>;
}

/// Internal trait for HashTable to get underlying vectors by underlying vector index
pub(in crate::core::structs::hash_table) trait HashVecInternal<V, const N: u64> {
    /// Returns reference to the underlying vector from the HashVector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * 'Option<&Vec<V>>' - Reference to the underlying vector, None if index was out of bounds
    fn get_vec(&self, index: u64) -> Option<&Vec<V>>;

    /// Returns mutable reference to the underlying vector from the HashVector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * 'Option<&mut Vec<V>>' - Mutable reference to the underlying vector, None if index was out of bounds
    fn get_vec_mut(&mut self, index: u64) -> Option<&mut Vec<V>>;
}

/// Common trait for HashTable to get statistics about underlying vectors
pub(in crate::core::structs::hash_table) trait HashVecStatisticsInternal<V, const N: u64> {
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