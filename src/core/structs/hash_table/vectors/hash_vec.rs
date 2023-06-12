
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
    fn have_item(&self, index: u64, value: V) -> (bool, usize);

    /// Finds value in the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to find
    /// # Returns
    /// * `Option<usize>` - Index of the value in the vector if it was found, None otherwise
    fn find_item(&self, index: u64, value: V) -> Option<usize>;

    /// Removes value from the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to remove
    fn remove(&mut self, index: u64, value: V);

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
    /// * `V` - Value that was removed
    fn remove_by_index(&mut self, index: u64, value_index: usize) -> V;

    /// Returns reference to the value from the HashVector by underlying vector index and value index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value_index` - Index of the value in the vector
    /// # Returns
    /// * `&V` - Reference to the value
    fn get_by_index(&self, index: u64, value_index: usize) -> &V;

    /// Returns mutable reference to the value from the HashVector by underlying vector index and value index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value_index` - Index of the value in the vector
    /// # Returns
    /// * `&mut V` - Mutable reference to the value
    fn get_by_index_mut(&mut self, index: u64, value_index: usize) -> &mut V;
}


pub(in crate::core::structs::hash_table) trait HashVecInternal<V, const N: u64> {
    /// Returns reference to the underlying vector from the HashVector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * `&Vec<V>` - Reference to the underlying vector
    fn get_vec(&self, index: u64) -> &Vec<V>;

    /// Returns mutable reference to the underlying vector from the HashVector by underlying vector index
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// # Returns
    /// * `&mut Vec<V>` - Mutable reference to the underlying vector
    fn get_vec_mut(&mut self, index: u64) -> &mut Vec<V>;
}