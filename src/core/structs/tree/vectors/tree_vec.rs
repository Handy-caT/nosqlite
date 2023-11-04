use crate::core::structs::tree::nodes::normalized_tree_index::NormalizedTreeIndex;
use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use crate::core::structs::tree::nodes::tree_node::TreeNode;
use std::ops::{Index, IndexMut};

/// The `TreeVec` trait is used to define the basic functions of a tree vector.
pub trait TreeVec<T>: Index<i32, Output = T> + IndexMut<i32> {
    /// Pushes a value to the vector.
    /// Returns the index of the added value.
    /// # Arguments
    /// * `value` - Value to push
    /// # Returns
    /// * `i32` - Index of the added value
    fn push(&mut self, value: T) -> i32;
    /// Returns the TreeNode at the given index.
    /// Returns `None` if the index is out of bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `Option<TreeNode<T>>` - TreeNode at the given index
    fn get(&self, index: i32) -> Option<TreeNode<T>>;
    /// Returns the mutable reference to the value at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&mut T` - Mutable reference to the value at the given index
    fn get_value_mut(&mut self, index: i32) -> &mut T;

    /// Removes the value at the given index.
    /// Returns the removed TreeNode.
    /// Returns `None` if the index is out of bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `Option<TreeNode<T>>` - Removed TreeNode
    fn remove(&mut self, index: i32) -> Option<TreeNode<T>>;
    /// Returns the length of the vector.
    /// # Returns
    /// * `usize` - Length of the vector
    fn len(&self) -> usize;
}

/// The `TreeVecIndexes` trait is used to define the basic functions of a index part of a tree vector.
pub trait TreeVecIndexes<T> {
    /// Returns the mutable reference to the TreeIndex at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&mut TreeIndex` - Mutable reference to the TreeIndex at the given index
    fn get_index_mut(&mut self, index: i32) -> &mut TreeIndex;
    /// Returns the reference to the TreeIndex at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&TreeIndex` - Reference to the TreeIndex at the given index
    fn get_index(&self, index: i32) -> &TreeIndex;
    /// Returns the mutable reference to the vector of TreeIndexes.
    /// # Returns
    /// * `&mut Vec<TreeIndex>` - Mutable reference to the vector of TreeIndexes
    fn get_indexes(&mut self) -> &mut Vec<TreeIndex>;
}

/// NormalizedTreeVecIndexes is used to define the basic functions of a index part of a normalized tree vector.
pub trait NormalizedTreeVecIndexes<T> {
    /// Returns the mutable reference to the NormalizedTreeIndex at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&mut NormalizedTreeIndex` - Mutable reference to the NormalizedTreeIndex at the given index
    fn get_index_mut(&mut self, index: i32) -> &mut NormalizedTreeIndex;
    /// Returns the reference to the NormalizedTreeIndex at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&NormalizedTreeIndex` - Reference to the NormalizedTreeIndex at the given index
    fn get_index(&self, index: i32) -> &NormalizedTreeIndex;
    /// Returns the mutable reference to the vector of NormalizedTreeIndexes.
    /// # Returns
    /// * `&mut Vec<NormalizedTreeIndex>` - Mutable reference to the vector of NormalizedTreeIndexes
    fn get_indexes(&mut self) -> &mut Vec<NormalizedTreeIndex>;
}

/// The `TreeVecLevels` trait is used to define the basic functions of a levels part of a tree vector.
/// It is used with vectors that uses tree level structure and allocates memory for each level when needed.
pub trait TreeVecLevels {
    /// Returns the count of allocated levels.
    /// # Returns
    /// * `u8` - Count of allocated levels
    fn get_allocated_levels(&self) -> u8;
    /// Returns the max length of the vector. After this length is reached, a new level is allocated.
    /// # Returns
    /// * `u64` - Max length of the vector
    fn get_max_length(&self) -> u64;
}

/// The 'BackwardTreeVec' trait is used to define the basic functions of tree with nodes that have a parent index.
pub trait BackwardTreeVec {
    /// Returns the parent index of the node at the given index.
    /// Returns `None` if the index is out of bounds.
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `Option<i32>` - Parent index of the node at the given index
    fn get_parent(&self, index: i32) -> Option<i32>;
    /// Sets the parent index of the node at the given index.
    /// # Arguments
    /// * `index` - Index of the node
    /// * `parent` - Parent index of the node
    fn add_parent(&mut self, index: i32, parent: i32);
}

/// The 'DefaultFunctions' trait is used to define the default functions of a tree vector.
/// This trait is private and should not be used outside of the crate.
pub(in crate::core::structs::tree::vectors) trait DefaultFunctions<T> {
    /// Returns the reference to the data vector.
    /// # Returns
    /// * `&Vec<T>` - Reference to the data vector
    fn get_data(&self) -> &Vec<T>;
    /// Returns the mutable reference to the data vector.
    /// # Returns
    /// * `&mut Vec<T>` - Mutable reference to the data vector
    fn get_data_mut(&mut self) -> &mut Vec<T>;

    /// Returns the reference to the empty vector.
    /// # Returns
    /// * `&Vec<u64>` - Reference to the empty vector
    fn get_empty(&self) -> &Vec<u64>;
    /// Returns the mutable reference to the empty vector.
    /// # Returns
    /// * `&mut Vec<u64>` - Mutable reference to the empty vector
    fn get_empty_mut(&mut self) -> &mut Vec<u64>;

    /// Returns the reference to the indexes vector.
    /// # Returns
    /// * `&Vec<TreeIndex>` - Reference to the indexes vector
    fn get_indexes(&self) -> &Vec<TreeIndex>;
    /// Returns the mutable reference to the indexes vector.
    /// # Returns
    /// * `&mut Vec<TreeIndex>` - Mutable reference to the indexes vector
    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex>;
}

/// The 'OptimizedFunctions' trait is used to define the optimized functions of a tree vector.
pub(in crate::core::structs::tree::vectors) trait OptimizedFunctions<T> {
    /// Returns the mutable reference to the allocated levels count.
    /// # Returns
    /// * `&mut u8` - Mutable reference to the allocated levels count
    fn get_allocated_levels_mut(&mut self) -> &mut u8;
    /// Returns the mutable reference to the max length.
    /// # Returns
    /// * `&mut u64` - Mutable reference to the max length
    fn get_max_length_mut(&mut self) -> &mut u64;

    /// Returns the length of the vector.
    /// # Returns
    /// * `u64` - Length of the vector
    fn get_length(&self) -> u64;
    /// Returns the mutable reference to the length of the vector.
    /// # Returns
    /// * `&mut u64` - Mutable reference to the length of the vector
    fn get_length_mut(&mut self) -> &mut u64;

    /// Function to allocate new level.
    fn allocate_level(&mut self);
}
