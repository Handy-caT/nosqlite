use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::tree_node::TreeNode;

pub trait TreeVec<T>: Index<i32, Output = T> + IndexMut<i32> {
    fn push(&mut self, value: T) -> i32;
    fn get(&mut self, index: i32) -> Option<TreeNode<T>>;
    fn get_value_mut(&mut self, index: i32) -> &mut T;
    fn get_index_mut(&mut self, index: i32) -> &mut TreeIndex;
    fn get_index(&self, index: i32) -> &TreeIndex;
    fn get_indexes(&mut self) -> &mut Vec<TreeIndex>;
    fn remove(&mut self, index: i32) -> Option<TreeNode<T>>;
    fn len(&self) -> usize;
}
