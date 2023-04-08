use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

const INITIAL_LEVELS: u8 = 6;

pub struct OptimizedTreeVec<T> {
    allocated_levels: u8,
    pub(in crate::core::structs::tree::vectors) max_length: u64,
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

    fn allocate_level(&mut self) {
        let new_length = 2u64.pow(self.allocated_levels as u32 + 1) - 1;
        let additional = new_length - self.max_length;

        self.data.reserve(additional as usize);
        self.indexes.reserve(additional as usize);

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

impl <T: Default + Copy> TreeVec<T> for OptimizedTreeVec<T> {
    fn push(&mut self, value: T) -> i32 {
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.length += 1;
            self.data.len() as u64
        };

        let indexes = TreeIndex::new_with_index(index as i32);

        if index == self.data.len() as u64 {
            if index == self.max_length {
                self.allocate_level();
            }
            self.data.push(value);
            self.indexes.push(indexes);
        } else {
            self.data[index as usize] = value;
            self.indexes[index as usize] = indexes;
        }

        index as i32
    }

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        let item = self.indexes.get(index as usize);
        return if item.is_none() {
            None
        } else {
            let item = item.unwrap();
            if item.index == -1 {
                None
            } else {
                let value = self.data.get(index as usize);
                Some(TreeNode {
                    value: *value.unwrap(),
                    indexes: *item,
                })
            }
        }
    }

    fn get_value_mut(&mut self, index: i32) -> &mut T {
        &mut self.data[index as usize]
    }

    fn get_index_mut(&mut self, index: i32) -> &mut TreeIndex {
        &mut self.indexes[index as usize]
    }


    fn get_indexes(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }

    fn get_index(&self, index: i32) -> &TreeIndex {
        &self.indexes[index as usize]
    }


    fn remove(&mut self, index: i32) -> Option<TreeNode<T>> {
        self.empty.push(index as u64);
        let mut item = self.indexes.get(index as usize);
        if item.is_none() {
            return None;
        }

        let item = *item.unwrap();
        if item.index == -1 {
            return None;
        }

        self.indexes[index as usize] = TreeIndex::default();

        if index as u64 == self.length - 1 {
            self.length -= 1;
        }

        let value = self.data.get(index as usize);

        Some(TreeNode {
            value: *value.unwrap(),
            indexes: item,
        })
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
    fn test_optimized_vec_is_empty() {
        let mut vec = OptimizedTreeVec::<i32>::new();
        let index = vec.push(1);

        assert_eq!(vec.is_empty_index(index), false);

        vec.remove(index);
        assert_eq!(vec.is_empty_index(index), true);
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
}