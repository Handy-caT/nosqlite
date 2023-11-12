use crate::core::structs::tree::{
    nodes::{TreeIndex, TreeNode},
    vectors::{
        tree_vec::{
            DefaultFunctions, OptimizedFunctions, TreeVec, TreeVecIndexes,
            TreeVecLevels,
        },
        vec_functions::{allocate_level, get, push, remove},
    },
};
use std::ops::{Index, IndexMut};

pub const INITIAL_LEVELS: u8 = 6;

/// Struct that represents a tree vector that is optimized for memory usage.
/// It allocates memory for a tree level when it is needed.
/// # Type parameters
/// * `T`: Type of the data that will be stored in the tree.
/// # Fields
/// * `allocated_levels`: Number of levels that are allocated in the tree.
/// * `max_length`: Maximum number of elements that can be stored in the tree.
/// * `length`: Number of elements that are stored in the tree.
/// * `data`: Vector that stores the data of the tree.
/// * `indexes`: Vector that stores the indexes of the tree.
/// * `empty`: Vector that stores the indexes of the empty nodes in the tree.
pub struct OptimizedTreeVec<T> {
    allocated_levels: u8,
    max_length: usize,
    length: usize,

    data: Vec<T>,
    indexes: Vec<TreeIndex>,
    empty: Vec<usize>,
}

impl<T: Default + Copy> OptimizedTreeVec<T> {
    /// Creates a new `OptimizedTreeVec<T>`.
    /// # Returns
    /// * `OptimizedTreeVec<T>`: New `OptimizedTreeVec<T>`.
    pub fn new() -> OptimizedTreeVec<T> {
        let mut vec = OptimizedTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
        };

        let length = 2usize.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length);
        vec.indexes.reserve(length);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }
}

impl<T> TreeVecLevels for OptimizedTreeVec<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> usize {
        self.max_length
    }
}

impl<T: Default + Copy> DefaultFunctions<T> for OptimizedTreeVec<T> {
    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    fn get_empty(&self) -> &Vec<usize> {
        &self.empty
    }

    fn get_empty_mut(&mut self) -> &mut Vec<usize> {
        &mut self.empty
    }

    fn get_indexes(&self) -> &Vec<TreeIndex> {
        &self.indexes
    }

    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl<T: Default + Copy> OptimizedFunctions<T> for OptimizedTreeVec<T> {
    fn get_allocated_levels_mut(&mut self) -> &mut u8 {
        &mut self.allocated_levels
    }

    fn get_max_length_mut(&mut self) -> &mut usize {
        &mut self.max_length
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn get_length_mut(&mut self) -> &mut usize {
        &mut self.length
    }

    fn allocate_level(&mut self) {
        allocate_level(self)
    }
}

impl<T: Default + Copy> TreeVecIndexes<T> for OptimizedTreeVec<T> {
    fn get_index_mut(&mut self, index: usize) -> &mut TreeIndex {
        &mut self.indexes[index]
    }

    fn get_index(&self, index: usize) -> &TreeIndex {
        &self.indexes[index]
    }

    fn get_indexes(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl<T: Default + Copy> TreeVec<T> for OptimizedTreeVec<T> {
    fn push(&mut self, value: T) -> usize {
        push(self, value)
    }

    fn get(&self, index: usize) -> Option<TreeNode<T>> {
        get(self, index)
    }

    fn get_value_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<TreeNode<T>> {
        remove(self, index)
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T: Default + Copy> Index<usize> for OptimizedTreeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Default + Copy> IndexMut<usize> for OptimizedTreeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_vec_new() {
        let vec = OptimizedTreeVec::<i32>::new();
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2usize.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_optimized_vec_add() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);
        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2usize.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_optimized_vec_get() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);
        let node = vec.get(index);

        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().value, 1);
    }

    #[test]
    fn test_optimized_vec_add_remove() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);
        vec.remove(index);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 1);

        let index = vec.push(1);
        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 0);

        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2usize.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_optimized_vec_remove() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);
        assert_eq!(vec.remove(index).is_none(), true);
    }

    #[test]
    fn test_optimized_vec_get_removed() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);
        assert_eq!(vec.get(index).is_none(), true);
    }

    #[test]
    fn test_optimized_vec_default_functions() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);
        assert_eq!(vec.get_value_mut(index), Some(&mut 1));
        assert_eq!(
            vec.get_index_mut(index),
            &mut TreeIndex::new_with_index(index)
        );
        assert_eq!(vec.get_indexes().len(), 1);
        assert_eq!(vec.get_index(index), &TreeIndex::new_with_index(index));
        assert_eq!(vec.len(), 1);
    }
}
