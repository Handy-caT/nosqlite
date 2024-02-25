use crate::core::structs::tree::{
    nodes::{TreeIndex, TreeNode},
    vectors::{
        tree_vec::{
            DefaultFunctions, Indexes, Levels, OptimizedFunctions, TreeVec,
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
#[derive(Debug)]
pub struct OptimizedTreeVec<T> {
    /// Number of allocated levels.
    allocated_levels: u8,

    /// Maximum length of the vector before the next level is allocated.
    max_length: usize,

    /// Length of the vector.
    length: usize,

    /// Vector that stores the data.
    data: Vec<T>,

    /// Vector that stores the indexes of the nodes.
    indexes: Vec<TreeIndex>,

    /// Vector that stores the empty indexes.
    empty: Vec<usize>,
}

impl<T> Levels for OptimizedTreeVec<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> usize {
        self.max_length
    }
}

impl<T: Default + Clone> DefaultFunctions<T> for OptimizedTreeVec<T> {
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

impl<T: Default + Clone> OptimizedFunctions<T> for OptimizedTreeVec<T> {
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
        allocate_level(self);
    }
}

impl<T: Default + Clone> Indexes<T> for OptimizedTreeVec<T> {
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

impl<T: Default + Clone> TreeVec<T> for OptimizedTreeVec<T> {
    fn new() -> OptimizedTreeVec<T> {
        let mut vec = OptimizedTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
        };

        let length = 2usize.pow(u32::from(INITIAL_LEVELS)) - 1;

        vec.data.reserve(length);
        vec.indexes.reserve(length);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }

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

impl<T: Default + Clone> Index<usize> for OptimizedTreeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Default + Clone> IndexMut<usize> for OptimizedTreeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Clone> Clone for OptimizedTreeVec<T> {
    fn clone(&self) -> Self {
        OptimizedTreeVec {
            allocated_levels: self.allocated_levels,
            max_length: self.max_length,
            length: self.length,
            data: self.data.clone(),
            indexes: self.indexes.clone(),
            empty: self.empty.clone(),
        }
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

        assert!(node.is_some());
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

        assert!(vec.remove(index).is_some());
        assert!(vec.remove(index).is_none());
    }

    #[test]
    fn test_optimized_vec_get_removed() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert!(vec.remove(index).is_some());
        assert!(vec.get(index).is_none());
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
