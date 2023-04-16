use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::tree_node::TreeNode;

pub trait TreeVec<T>: Index<i32, Output = T> + IndexMut<i32> {
    fn push(&mut self, value: T) -> i32;
    fn get(&mut self, index: i32) -> Option<TreeNode<T>>;

    fn remove(&mut self, index: i32) -> Option<TreeNode<T>>;
    fn len(&self) -> usize;
}

pub trait TreeVecIndexes<T> {
    fn get_value_mut(&mut self, index: i32) -> &mut T;
    fn get_index_mut(&mut self, index: i32) -> &mut TreeIndex;
    fn get_index(&self, index: i32) -> &TreeIndex;
    fn get_indexes(&mut self) -> &mut Vec<TreeIndex>;
}

pub trait TreeVecLevels {
    fn get_allocated_levels(&self) -> u8;
    fn get_max_length(&self) -> u64;
}

pub trait BackwardTreeVec {
    fn get_parent(&self, index: i32) -> Option<i32>;
    fn add_parent(&mut self, index: i32, parent: i32);
}

pub(in crate::core::structs::tree::vectors) trait DefaultFunctions<T> {
    fn get_data(&self) -> &Vec<T>;
    fn get_data_mut(&mut self) -> &mut Vec<T>;
    
    fn get_empty(&self) -> &Vec<u64>;
    fn get_empty_mut(&mut self) -> &mut Vec<u64>;
    
    fn get_indexes(&self) -> &Vec<TreeIndex>;
    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex>;
}

pub(in crate::core::structs::tree::vectors) trait OptimizedFunctions<T> {
    fn get_allocated_levels_mut(&mut self) -> &mut u8;

    fn get_max_length_mut(&mut self) -> &mut u64;
    
    fn get_length(&self) -> u64;
    fn get_length_mut(&mut self) -> &mut u64;

    fn allocate_level(&mut self);
}