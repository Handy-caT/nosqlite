use crate::core::structs::tree::vectors::tree_vec::TreeVec;

/// TreeObject is a trait that defines the basic operations that a tree object must implement.
pub trait TreeObject<T> {
    /// Pushes a value into the tree. Returns the index of the value.
    fn push(&mut self, value: T) -> i32;
    /// Finds a value in the tree. Returns the index of the value.
    /// If the value is not found, returns None.
    fn find(&mut self, value: T) -> Option<i32>;
    /// Removes a value from the tree. Returns the value.
    /// If the value is not found, returns None.
    fn remove_by_value(&mut self, value: T) -> Option<T>;
    /// Checks if the object is empty.
    fn is_empty(&self) -> bool;
    /// Returns the length of the object.
    fn len(&self) -> usize;
}

/// TreeObjectVec is a trait that defines the vector operations that a tree object can implement.
pub trait TreeObjectVec<T, M: TreeVec<T> + Sized> {
    /// Returns the value of the index in underlying vector.
    /// If the index is wrong, returns None.
    fn get(&mut self, index: i32) -> Option<T>;
    /// Returns the mutable reference to the underlying vector.
    fn get_nodes_mut(&mut self) -> &mut M;
    /// Returns the reference to the underlying vector.
    fn get_nodes(&self) -> &M;
    /// Returns thr root index of the tree.
    fn get_root_index(&self) -> i32;
    /// Removes item from the tree by index. Returns the value.
    /// If the index is wrong, returns None.
    fn remove_by_index(&mut self, index: i32) -> Option<T>;
}

/// TreeObjectFind is a trait that defines the find operations that a tree object can implement.
/// It expands find features of the TreeObject trait.
pub trait TreeObjectFind<T> {
    /// Finds the first value that is greater than the given value or equal to it.
    /// Returns the index of the value and the value itself.
    fn find_greater_equal(&mut self, value: T) -> Option<(i32,T)>;
    /// Finds the first value that is less than the given value or equal to it.
    /// Returns the index of the value and the value itself.
    fn find_less_equal(&mut self, value: T) -> Option<(i32,T)>;
}
