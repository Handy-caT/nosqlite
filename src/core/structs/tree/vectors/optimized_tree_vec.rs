use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

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

    fn is_empty_index(&self, index: i32) -> bool {
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

impl <T: Default + Copy + PartialEq> TreeVec<T> for OptimizedTreeVec<T> {
    fn push(&mut self, value: T) -> i32 {
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

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        let item = self.data.get(index as usize);
        return if item.is_none() {
            None
        } else {
            let item = item.unwrap();
            if *item == TreeNode::default() {
                if self.is_empty_index(index) {
                    None
                } else {
                    Some(*item)
                }
            } else {
                Some(*item)
            }
        }
    }

    fn remove(&mut self, index: i32) -> Option<TreeNode<T>> {
        self.empty.push(index as u64);
        let mut item = self.data.get(index as usize);
        if item.is_none() {
            return None;
        }

        let item = *item.unwrap();

        self.data[index as usize] = TreeNode::default();

        if index as u64 == self.length - 1 {
            self.length -= 1;
        }

        Some(item)
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
    fn test_optimized_vec_is_empty() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);

        assert_eq!(vec.is_empty_index(index), false);

        vec.remove(index);
        assert_eq!(vec.is_empty_index(index), true);
    }
}