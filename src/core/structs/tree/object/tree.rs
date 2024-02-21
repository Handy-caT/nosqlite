use crate::core::structs::tree::vectors::tree_vec::TreeVec;
use std::cmp::Ordering;

/// [`Tree`] is a trait that defines the basic operations that
/// a tree object must implement.
pub trait Tree<T> {
    /// Creates new [`Tree`] using specified vector.
    /// Vector must implement [`TreeVec`] trait.
    /// # Arguments
    /// * `vec` - Vector to be used as a tree.
    /// # Returns
    /// * `BalancedTree<T, M>` - New [`Tree`].
    fn new() -> Self;

    /// Creates new [`Tree`] using specified vector and compare function
    /// # Arguments
    /// * `vec` - Vector to be used as a tree.
    /// * `compare` - Compare function.
    /// # Returns
    /// * `BalancedTree<T, M>` - New [`Tree`].
    fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self;

    /// Pushes a value into the tree. Returns the index of the value.
    /// # Arguments
    /// * `value` - The value to be pushed.
    /// # Returns
    /// * `usize` - The index of the value.
    fn push(&mut self, value: T) -> usize;

    /// Finds a value in the tree. Returns the index of the value.
    /// If the value is not found, returns None.
    /// # Arguments
    /// * `value` - The value to be found.
    /// # Returns
    /// * `Option<usize>` - The index of the value.
    fn find(&mut self, value: &T) -> Option<usize>;

    /// Removes a value from the tree. Returns the value.
    /// If the value is not found, returns None.
    /// # Arguments
    /// * `value` - The value to be removed.
    /// # Returns
    /// * `Option<T>` - The value.
    fn remove_by_value(&mut self, value: &T) -> Option<T>;

    /// Removes some object from a tree
    /// # Returns
    /// /// * `Option<T>` - The value, None if empty
    fn pop(&self) -> Option<T>;

    /// Checks if the object is empty.
    /// # Returns
    /// * `bool` - True if the object is empty, false otherwise.
    fn is_empty(&self) -> bool;

    /// Returns the length of the object.
    /// # Returns
    /// * `usize` - The length of the object.
    fn len(&self) -> usize;
}

/// [`VecFunctions`] is a trait that defines the vector operations that
/// a tree object can implement.
pub trait VecFunctions<T, M: TreeVec<T> + Sized> {
    /// Returns the value of the index in underlying vector.
    /// If the index is wrong, returns None.
    /// # Arguments
    /// * `index` - The index of the value.
    /// # Returns
    /// * `Option<T>` - The value.
    fn get(&mut self, index: usize) -> Option<T>;
    /// Returns the mutable reference to the underlying vector.
    /// # Returns
    /// * `&mut M` - The mutable reference to the underlying vector.
    fn get_nodes_mut(&mut self) -> &mut M;
    /// Returns the reference to the underlying vector.
    /// # Returns
    /// * `&M` - The reference to the underlying vector.
    fn get_nodes(&self) -> &M;
    /// Returns thr root index of the tree.
    /// # Returns
    /// * `Option<usize>` - The root index of the tree.
    fn get_root_index(&self) -> Option<usize>;
    /// Removes item from the tree by index. Returns the value.
    /// If the index is wrong, returns None.
    /// # Arguments
    /// * `index` - The index of the value.
    /// # Returns
    /// * `Option<T>` - The value.
    fn remove_by_index(&mut self, index: usize) -> Option<T>;
}

/// [`FindFunctions`] is a trait that defines the find operations
/// that a tree object can implement.
/// It expands find features of the [`Tree`] trait.
pub trait FindFunctions<T> {
    /// Finds the first value that is greater than the given value
    /// or equals to it.
    /// Returns the index of the value and the value itself.
    /// # Arguments
    /// * `value` - The value to be found.
    /// # Returns
    /// * `Option<(usize,T)>` - The index of the value and the value itself.
    fn find_greater_equal(&mut self, value: &T) -> Option<(usize, T)>;
    /// Finds the first value that is less than the given value or equal to it.
    /// Returns the index of the value and the value itself.
    /// # Arguments
    /// * `value` - The value to be found.
    /// # Returns
    /// * `Option<(usize,T)>` - The index of the value and the value itself.
    fn find_less_equal(&mut self, value: &T) -> Option<(usize, T)>;
}
