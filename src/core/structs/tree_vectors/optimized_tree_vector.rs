use std::ops::{Index, IndexMut};
use crate::core::structs::tree_nodes::tree_node::TreeNode;
use crate::core::structs::tree_vectors::tree_vec::TreeVec;

const INITIAL_LEVELS: u8 = 6;

pub struct OptimizedTreeVec<T> {
    allocated_levels: u8,
    max_length: u64,
    length: u64,
    data: Vec<TreeNode<T>>,
    empty: Vec<u64>,
}

impl <T: Default + Copy> OptimizedTreeVec<T> {
    pub fn new() -> OptimizedTreeVec<T> {
        let mut vec = OptimizedTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
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

    pub(in crate::core::structs) fn peek_node(&self, index: i32) -> Option<TreeNode<T>> {
        if index < 0 {
            return None;
        } else if index as u64 >= self.data.len() as u64 {
            return None;
        } else {
            let node = self.data[index as usize];
            Some(node)
        }
    }

    pub(in crate::core::structs) fn is_empty_index(&self, index: i32) -> bool {
        return if index < 0 {
            true
        } else if index as u64 >= self.data.len() as u64 {
            true
        } else {
            if self.empty.contains(&(index as u64)) {
                true
            } else {
                false
            }
        }
    }
}

impl <T: Default + Copy> TreeVec<T> for OptimizedTreeVec<T> {
    fn add(&mut self, value: T) -> i32 {
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.length += 1;
            self.data.len() as u64
        };

        let node = TreeNode::new_with_index(value, index as i32);

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

        if index as u64 == self.length - 1 {
            self.length -= 1;
        }
    }

    fn len(&self) -> usize {
        self.length as usize
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
        let index = vec.add(1);
        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.allocated_levels, INITIAL_LEVELS);
        assert_eq!(vec.max_length, 2u64.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_optimized_vec_get() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.add(1);
        let node = vec.get(index);
        assert_eq!(node.value, 1);
    }

    #[test]
    fn test_optimized_vec_peek() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.add(1);
        let node = vec.peek_node(index);

        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().value, 1);
    }


    #[test]
    fn test_optimized_vec_add_remove() {
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

    #[test]
    fn test_optimized_vec_is_empty() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.add(1);

        assert_eq!(vec.is_empty_index(index), false);

        vec.remove(index);
        assert_eq!(vec.is_empty_index(index), true);
    }
}