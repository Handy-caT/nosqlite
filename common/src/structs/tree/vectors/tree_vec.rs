use crate::structs::tree::nodes::{
    normalized_tree_index::NormalizedTreeIndex, tree_index::TreeIndex,
    tree_node::TreeNode,
};
use std::ops::{Index, IndexMut};

/// The [`TreeVec`] trait is used to define the basic functions
/// of a tree vector.
pub trait TreeVec<T>: Index<usize, Output = T> + IndexMut<usize> {
    /// Creates a new instance of the [`TreeVec`].
    /// # Returns
    /// * `Self` - New instance of the [`TreeVec`]
    fn new() -> Self;

    /// Pushes a value to the vector.
    /// Returns the index of the added value.
    /// # Arguments
    /// * `value` - Value to push
    /// # Returns
    /// * `usize` - Index of the added value
    fn push(&mut self, value: T) -> usize;

    /// Returns the [`TreeNode`] at the given index.
    /// Returns `None` if the index is out of bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `Option<TreeNode<T>>` - [`TreeNode`] at the given index
    fn get(&self, index: usize) -> Option<TreeNode<T>>;

    /// Returns the mutable reference to the value at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&mut T` - Mutable reference to the value at the given index
    fn get_value_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Removes the value at the given index.
    /// Returns the removed [`TreeNode`].
    /// Returns `None` if the index is out of bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `Option<TreeNode<T>>` - Removed [`TreeNode`]
    fn remove(&mut self, index: usize) -> Option<TreeNode<T>>;

    /// Returns the length of the vector.
    /// # Returns
    /// * `usize` - Length of the vector
    fn len(&self) -> usize;
}

/// The [`Indexes`] trait is used to define the basic functions of a
/// index part of a tree vector.
pub trait Indexes<T> {
    /// Returns the mutable reference to the [`TreeIndex`] at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&mut TreeIndex` - Mutable reference to the
    /// [`TreeIndex`] at the given index
    fn get_index_mut(&mut self, index: usize) -> &mut TreeIndex;

    /// Returns the reference to the [`TreeIndex`] at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&TreeIndex` - Reference to the [`TreeIndex`] at the given index
    fn get_index(&self, index: usize) -> &TreeIndex;

    /// Returns the mutable reference to the vector of [`TreeIndexes`].
    /// # Returns
    /// * `&mut Vec<TreeIndex>` - Mutable reference to the
    /// vector of [`TreeIndexes`]
    fn get_indexes(&mut self) -> &mut Vec<TreeIndex>;
}

/// [`NormalizedIndexes`] is used to define the basic functions
/// of a index part of a normalized tree vector.
pub trait NormalizedIndexes<T> {
    /// Returns the mutable reference to the [`NormalizedTreeIndex`]
    /// at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&mut NormalizedTreeIndex` - Mutable reference to the
    /// [`NormalizedTreeIndex`] at the given index
    fn get_index_mut(&mut self, index: usize) -> &mut NormalizedTreeIndex;

    /// Returns the reference to the [`NormalizedTreeIndex`] at the given index.
    /// Index must be in bounds.
    /// # Arguments
    /// * `index` - Index of the value
    /// # Returns
    /// * `&NormalizedTreeIndex` - Reference to the [`NormalizedTreeIndex`]
    /// at the given index
    fn get_index(&self, index: usize) -> &NormalizedTreeIndex;

    /// Returns the mutable reference to the vector of
    /// [`NormalizedTreeIndex`]es.
    /// # Returns
    /// * `&mut Vec<NormalizedTreeIndex>` - Mutable reference to the vector
    /// of [`NormalizedTreeIndexes`]
    fn get_indexes(&mut self) -> &mut Vec<NormalizedTreeIndex>;
}

/// The [`Levels`] trait is used to define the basic functions of a levels
/// part of a tree vector.
/// It is used with vectors that uses tree level structure and allocates memory
/// for each level when needed.
pub trait Levels {
    /// Returns the count of allocated levels.
    /// # Returns
    /// * `u8` - Count of allocated levels
    fn get_allocated_levels(&self) -> u8;

    /// Returns the max length of the vector. After this length is reached,
    /// a new level is allocated.
    /// # Returns
    /// * `u64` - Max length of the vector
    fn get_max_length(&self) -> usize;
}

/// The [`Backward`] trait is used to define the basic functions
/// of tree with nodes that have a parent index.
pub trait Backward {
    /// Returns the parent index of the node at the given index.
    /// Returns `None` if the index is out of bounds.
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `Option<usize>` - Parent index of the node at the given index
    fn get_parent(&self, index: usize) -> Option<usize>;

    /// Sets the parent index of the node at the given index.
    /// # Arguments
    /// * `index` - Index of the node
    /// * `parent` - Parent index of the node
    fn add_parent(&mut self, index: usize, parent: usize) -> Option<()>;
}

/// The [`DefaultFunctions`] trait is used to define
/// the default functions of a tree vector.
/// This trait is private and should not be used outside of the crate.
pub trait DefaultFunctions<T> {
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
    /// * `&Vec<usize>` - Reference to the empty vector
    fn get_empty(&self) -> &Vec<usize>;

    /// Returns the mutable reference to the empty vector.
    /// # Returns
    /// * `&mut Vec<usize>` - Mutable reference to the empty vector
    fn get_empty_mut(&mut self) -> &mut Vec<usize>;

    /// Returns the reference to the indexes vector.
    /// # Returns
    /// * `&Vec<TreeIndex>` - Reference to the indexes vector
    fn get_indexes(&self) -> &Vec<TreeIndex>;

    /// Returns the mutable reference to the indexes vector.
    /// # Returns
    /// * `&mut Vec<TreeIndex>` - Mutable reference to the indexes vector
    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex>;
}

/// The [`OptimizedFunctions`] trait is used to define
/// the optimized functions of a tree vector.
pub trait OptimizedFunctions<T> {
    /// Returns the mutable reference to the allocated levels count.
    /// # Returns
    /// * `&mut u8` - Mutable reference to the allocated levels count
    fn get_allocated_levels_mut(&mut self) -> &mut u8;

    /// Returns the mutable reference to the max length.
    /// # Returns
    /// * `&mut usize` - Mutable reference to the max length
    fn get_max_length_mut(&mut self) -> &mut usize;

    /// Returns the length of the vector.
    /// # Returns
    /// * `usize` - Length of the vector
    fn get_length(&self) -> usize;

    /// Returns the mutable reference to the length of the vector.
    /// # Returns
    /// * `&mut usize` - Mutable reference to the length of the vector
    fn get_length_mut(&mut self) -> &mut usize;

    /// Function to allocate new level.
    fn allocate_level(&mut self);
}
