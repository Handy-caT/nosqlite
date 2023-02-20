use std::ops::{Index, IndexMut};
use crate::core::tree::tree_node::TreeNode;

pub trait TreeVec<T>: Index<u64, Output = TreeNode<T>> + IndexMut<u64, Output = TreeNode<T>> {
    fn add(&mut self, value: T) -> i32;
    fn get(&mut self, index: u64) -> TreeNode<T>;
    fn swap(&mut self, index1: u64, index2: u64);
    fn remove(&mut self, index: u64);
    fn len(&self) -> usize;
}

pub struct DefaultTreeVec<T> {
    data: Vec<TreeNode<T>>,
    empty: Vec<u64>,
}

impl <T: Default + Copy> TreeVec<T> for DefaultTreeVec<T> {
    fn add(&mut self, value: T) -> i32 {
        let node = TreeNode::new(value);
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.data.len() as u64
        };

        if index == self.data.len() as u64 {
            self.data.push(node);
        } else {
            self.data[index as usize] = node;
        }

        index as i32
    }

    fn get(self: &mut DefaultTreeVec<T>, index: u64) -> TreeNode<T> {
        self.data[index as usize]
    }

    fn swap(&mut self, index1: u64, index2: u64) {
        let node1 = self.data[index1 as usize];
        let node2 = self.data[index2 as usize];
        self.data[index1 as usize] = node2;
        self.data[index2 as usize] = node1;
    }

    fn remove(&mut self, index: u64) {
        self.empty.push(index);
        self.data[index as usize] = TreeNode::default();
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl <T: Default + Copy> DefaultTreeVec<T> {
    pub fn new() -> DefaultTreeVec<T> {
        DefaultTreeVec {
            data: Vec::new(),
            empty: Vec::new(),
        }
    }
}


impl <T> Index<u64> for DefaultTreeVec<T> {
    type Output = TreeNode<T>;

    fn index(&self, index: u64) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl <T> IndexMut<u64> for DefaultTreeVec<T> {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree_vec = DefaultTreeVec::<u64>::new();
        assert_eq!(tree_vec.data.len(), 0);
        assert_eq!(tree_vec.empty.len(), 0);
    }

    #[test]
    fn test_add() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.add(1);
        assert_eq!(tree_vec.data.len(), 1);
        assert_eq!(tree_vec.empty.len(), 0);
        assert_eq!(tree_vec.data[0].value, 1);
    }

    #[test]
    fn test_get() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.add(1);
        assert_eq!(tree_vec.get(0).value, 1);
    }

    #[test]
    fn test_remove() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.add(1);
        tree_vec.remove(0);
        assert_eq!(tree_vec.data.len(), 1);
        assert_eq!(tree_vec.empty.len(), 1);
        assert_eq!(tree_vec.empty[0], 0);
    }

    #[test]
    fn test_add_remove() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.add(1);
        tree_vec.add(2);
        tree_vec.add(3);
        tree_vec.remove(1);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 1);
        assert_eq!(tree_vec.empty[0], 1);

        tree_vec.add(6);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 0);

        assert_eq!(tree_vec.data[0].value, 1);
        assert_eq!(tree_vec.data[1].value, 6);
        assert_eq!(tree_vec.data[2].value, 3);
    }

}