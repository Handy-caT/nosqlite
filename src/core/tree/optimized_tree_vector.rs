use std::ops::{Index, IndexMut};
use crate::core::tree::tree_node::TreeNode;
use crate::core::tree::tree_vec::TreeVec;

const INITIAL_LEVELS: u8 = 6;

pub struct OptimizedTreeVec<T> {
    allocated_levels: u8,
    max_length: u64,
    data: Vec<TreeNode<T>>,
    empty: Vec<u64>,
}

impl <T: Default + Copy> OptimizedTreeVec<T> {
    pub fn new() -> OptimizedTreeVec<T> {
        let mut vec = OptimizedTreeVec {
            allocated_levels: 0,
            max_length: 0,
            data: Vec::new(),
            empty: Vec::new(),
        };

        let length = 2u64.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length as usize);
        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }

    fn allocate_level(&mut self) {
        let new_length = 2u64.pow(self.allocated_levels as u32 + 1) - 1;
        let additional = new_length - self.max_length;

        self.data.reserve(additional as usize);
        self.max_length = new_length;
        self.allocated_levels += 1;
    }
}

impl <T> Index<u64> for OptimizedTreeVec<T> {
    type Output = TreeNode<T>;

    fn index(&self, index: u64) -> &TreeNode<T> {
        &self.data[index as usize]
    }
}

impl <T> IndexMut<u64> for OptimizedTreeVec<T> {
    fn index_mut(&mut self, index: u64) -> &mut TreeNode<T> {
        &mut self.data[index as usize]
    }
}

impl <T: Default + Copy> TreeVec<T> for OptimizedTreeVec<T> {
    fn add(&mut self, value: T) -> i32 {
        let node = TreeNode::new(value);
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.data.len() as u64
        };

        if index == self.data.len() as u64 {
            if index == self.max_length {
                self.allocate_level();
            }
            self.data.push(node);
        } else {
            self.data[index as usize] = node;
        }

        index as i32
    }

    fn get(self: &mut OptimizedTreeVec<T>, index: i32) -> TreeNode<T> {
        self.data[index as usize]
    }

    fn swap(&mut self, index1: i32, index2: i32) {
        let node1 = self.data[index1 as usize];
        let node2 = self.data[index2 as usize];
        self.data[index1 as usize] = node2;
        self.data[index2 as usize] = node1;
    }

    fn remove(&mut self, index: i32) {
        self.empty.push(index as u64);
        self.data[index as usize] = TreeNode::default();
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec = OptimizedTreeVec::<i32>::new();
        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_add() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.add(1);
        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_get() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.add(1);
        let node = vec.get(index);
        assert_eq!(node.value, 1);
    }

    #[test]
    fn test_add_remove() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.add(1);
        vec.remove(index);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 1);

        let index = vec.add(1);
        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 0);

        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
    }
}