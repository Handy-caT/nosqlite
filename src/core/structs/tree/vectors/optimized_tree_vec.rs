use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::tree_vec::{DefaultFunctions, OptimizedFunctions, TreeVec, TreeVecIndexes, TreeVecLevels};
use crate::core::structs::tree::vectors::vec_functions::{allocate_level, get, push, remove};

pub const INITIAL_LEVELS: u8 = 6;

pub struct OptimizedTreeVec<T> {
    allocated_levels: u8,
    max_length: u64,
    length: u64,

    data: Vec<T>,
    indexes: Vec<TreeIndex>,
    empty: Vec<u64>,
}

impl <T: Default + Copy> OptimizedTreeVec<T> {
    pub fn new() -> OptimizedTreeVec<T> {
        let mut vec = OptimizedTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
        };

        let length = 2u64.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length as usize);
        vec.indexes.reserve(length as usize);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }
}

impl <T> TreeVecLevels for OptimizedTreeVec<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> u64 {
        self.max_length
    }
}

impl  <T: Default + Copy> DefaultFunctions<T> for OptimizedTreeVec<T> {
    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    fn get_empty(&self) -> &Vec<u64> {
        &self.empty
    }

    fn get_empty_mut(&mut self) -> &mut Vec<u64> {
        &mut self.empty
    }

    fn get_indexes(&self) -> &Vec<TreeIndex> {
        &self.indexes
    }

    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl  <T: Default + Copy> OptimizedFunctions<T> for OptimizedTreeVec<T> {

    fn get_allocated_levels_mut(&mut self) -> &mut u8 {
        &mut self.allocated_levels
    }

    fn get_max_length_mut(&mut self) -> &mut u64 {
        &mut self.max_length
    }

    fn get_length(&self) -> u64 {
        self.length
    }

    fn get_length_mut(&mut self) -> &mut u64 {
        &mut self.length
    }

    fn allocate_level(&mut self) {
        allocate_level(self)
    }
}

impl <T: Default + Copy> TreeVecIndexes<T> for OptimizedTreeVec<T> {
    fn get_value_mut(&mut self, index: i32) -> &mut T {
        &mut self.data[index as usize]
    }

    fn get_index_mut(&mut self, index: i32) -> &mut TreeIndex {
        &mut self.indexes[index as usize]
    }


    fn get_index(&self, index: i32) -> &TreeIndex {
        &self.indexes[index as usize]
    }

    fn get_indexes(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl <T: Default + Copy> TreeVec<T> for OptimizedTreeVec<T> {
    fn push(&mut self, value: T) -> i32 {
        push(self, value)
    }

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        get(self, index)
    }

    fn remove(&mut self, index: i32) -> Option<TreeNode<T>> {
        remove(self, index)
    }

    fn len(&self) -> usize {
        self.length as usize
    }
}

impl <T: Default + Copy> Index<i32> for OptimizedTreeVec<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl <T: Default + Copy> IndexMut<i32> for OptimizedTreeVec<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.data[index as usize]
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
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_optimized_vec_add() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);
        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
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
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
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
        assert_eq!(vec.get_value_mut(index), &mut 1);
        assert_eq!(vec.get_index_mut(index), &mut TreeIndex::new_with_index(index));
        assert_eq!(vec.get_indexes().len(), 1);
        assert_eq!(vec.get_index(index), &TreeIndex::new_with_index(index));
        assert_eq!(vec.len(), 1);
    }
}