use std::ops::{Index, IndexMut};
use crate::core::structs::tree_nodes::tree_node::TreeNode;

pub trait TreeVec<T>: Index<u64, Output = TreeNode<T>> + IndexMut<u64, Output = TreeNode<T>> {
    fn add(&mut self, value: T) -> i32;
    fn get(&mut self, index: i32) -> TreeNode<T>;
    fn swap(&mut self, index1: i32, index2: i32);
    fn remove(&mut self, index: i32);
    fn len(&self) -> usize;
}
