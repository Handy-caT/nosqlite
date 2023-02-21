use std::ops::{Index, IndexMut};
use crate::core::tree::tree_node::TreeNode;

pub trait TreeVec<T>: Index<u64, Output = TreeNode<T>> + IndexMut<u64, Output = TreeNode<T>> {
    fn add(&mut self, value: T) -> i32;
    fn get(&mut self, index: u64) -> TreeNode<T>;
    fn swap(&mut self, index1: u64, index2: u64);
    fn remove(&mut self, index: u64);
    fn len(&self) -> usize;
}
