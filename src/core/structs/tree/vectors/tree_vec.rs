use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_node::TreeNode;

pub trait TreeVec<T>: Index<usize, Output = TreeNode<T>> + IndexMut<usize, Output = TreeNode<T>> {
    fn push(&mut self, value: T) -> i32;
    //fn pop(&mut self) -> Option<TreeNode<T>>;
    fn get(&mut self, index: i32) -> Option<TreeNode<T>>;
    //fn swap(&mut self, index1: i32, index2: i32);
    fn remove(&mut self, index: i32) -> Option<TreeNode<T>>;
    fn len(&self) -> usize;
}
