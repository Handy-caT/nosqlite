
/// Common trait for HashTable data
pub trait HashVec<V, const N: u64> {
    /// Adds value to the HashVector by underlying vector index and value itself
    /// # Arguments
    /// * `index` - Index of the underlying vector
    /// * `value` - Value to add
    /// # Returns
    /// * `(u64, usize)` - Index of the underlying vector and index of the value in the vector
    fn push(&mut self, index: u64, value: V) -> (u64, usize);
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