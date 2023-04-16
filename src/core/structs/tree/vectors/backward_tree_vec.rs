use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
use crate::core::structs::tree::vectors::tree_vec::{BackwardTreeVec, DefaultFunctions, OptimizedFunctions, TreeVec, TreeVecLevels};
use crate::core::structs::tree::vectors::vec_functions::{allocate_level, get, push, remove};

pub struct BackwardsTreeVec<T> {
    allocated_levels: u8,
    max_length: u64,
    length: u64,

    data: Vec<T>,
    indexes: Vec<TreeIndex>,
    empty: Vec<u64>,

    parents: Vec<u64>,
}

impl <T: Default + Copy> BackwardsTreeVec<T> {
    pub fn new() -> BackwardsTreeVec<T> {
        let mut vec = BackwardsTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
            parents: Vec::new(),
        };

        let length = 2u64.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length as usize);
        vec.indexes.reserve(length as usize);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }
}

impl <T: Default + Copy> TreeVecLevels for BackwardsTreeVec<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> u64 {
        self.max_length
    }
}

impl <T: Default + Copy> DefaultFunctions<T> for BackwardsTreeVec<T> {
    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    fn get_empty(&self) -> &Vec<u64> {
        &self.empty
    }

    fn get_empty_mut(&mut self) -> &mut Vec<u64> {
        &mut self.empty
    }

    fn get_indexes(&self) -> &Vec<TreeIndex> {
        &self.indexes
    }

    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl <T: Default + Copy> OptimizedFunctions<T> for BackwardsTreeVec<T> {
    fn get_allocated_levels_mut(&mut self) -> &mut u8 {
        &mut self.allocated_levels
    }

    fn get_max_length_mut(&mut self) -> &mut u64 {
        &mut self.max_length
    }

    fn get_length(&self) -> u64 {
        self.length
    }

    fn get_length_mut(&mut self) -> &mut u64 {
        &mut self.length
    }

    fn allocate_level(&mut self) {
        allocate_level(self)
    }
}

impl <T: Default + Copy> TreeVec<T> for BackwardsTreeVec<T> {
    fn push(&mut self, value: T) -> i32 {
        let index = push(self, value);
        if index == (self.length - 1) as i32 {
            self.parents.push(0);
        }

        index
    }

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        get(self, index)
    }

    fn remove(&mut self, index: i32) -> Option<TreeNode<T>> {
        remove(self, index)
    }

    fn len(&self) -> usize {
        self.length as usize
    }
}

impl <T: Default + Copy> Index<i32> for BackwardsTreeVec<T> {
    type Output = T;

    fn index(&self, index: i32) -> &T {
        &self.data[index as usize]
    }
}

impl <T: Default + Copy> IndexMut<i32> for BackwardsTreeVec<T> {
    fn index_mut(&mut self, index: i32) -> &mut T {
        &mut self.data[index as usize]
    }
}

impl <T: Default + Copy> BackwardTreeVec for BackwardsTreeVec<T> {
    fn get_parent(&self, index: i32) -> Option<i32> {
        if index == 0 {
            return None;
        }

        let parent = self.parents[index as usize];

        if parent == 0 {
            return None;
        }

        Some(parent as i32)
    }

    fn add_parent(&mut self, index: i32, parent: i32) {
        self.parents[index as usize] = parent as u64;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::vectors::backward_tree_vec::BackwardsTreeVec;
    use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
    use crate::core::structs::tree::vectors::tree_vec::{DefaultFunctions, TreeVec, TreeVecLevels};

    #[test]
    fn test_backwards_tree_vec_new() {
        let vec = BackwardsTreeVec::<i32>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(vec.get_max_length(), 2u64.pow(INITIAL_LEVELS as u32) - 1);
        assert_eq!(vec.get_data().len(), 0);
        assert_eq!(vec.get_indexes().len(), 0);
        assert_eq!(vec.get_empty().len(), 0);
        assert_eq!(vec.parents.len(), 0);
    }

    #[test]
    fn test_backwards_tree_vec_push() {
        let mut vec = BackwardsTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(vec.get_max_length(), 2u64.pow(INITIAL_LEVELS as u32) - 1);
        assert_eq!(vec.get_data().len(), 3);
        assert_eq!(vec.get_indexes().len(), 3);
        assert_eq!(vec.get_empty().len(), 0);
        assert_eq!(vec.parents.len(), 3);
    }

    #[test]
    fn test_backwards_tree_vec_remove() {
        let mut vec = BackwardsTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        let node = vec.remove(1).unwrap();

        assert_eq!(node.value, 2);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get_data().len(), 3);
        assert_eq!(vec.get_indexes().len(), 3);
        assert_eq!(vec.get_empty().len(), 1);
        assert_eq!(vec.parents.len(), 3);
    }
}

