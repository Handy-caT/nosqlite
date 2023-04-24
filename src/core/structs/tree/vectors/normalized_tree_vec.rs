use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
use crate::core::structs::tree::vectors::tree_vec::{DefaultFunctions, OptimizedFunctions, TreeVec, TreeVecLevels};

pub struct NormalizedTreeVector<T> {
    allocated_levels: u8,
    max_length: u64,
    length: u64,

    data: Vec<T>,
    indexes: Vec<i32>,
}

impl <T: Default + Copy> NormalizedTreeVector<T> {
    pub fn new() -> NormalizedTreeVector<T> {
        let mut vec = NormalizedTreeVector {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
        };

        let length = 2u64.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length as usize);
        vec.indexes.reserve(length as usize);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }
}

impl <T> TreeVecLevels for NormalizedTreeVector<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> u64 {
        self.max_length
    }
}

impl <T: Default + Copy> OptimizedFunctions<T> for NormalizedTreeVector<T> {
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
        let length = 2u64.pow(self.allocated_levels as u32) - 1;

        self.data.reserve(length as usize);
        self.indexes.reserve(length as usize);

        self.max_length += length;
        self.allocated_levels += 1;
    }
}

impl <T: Default + Copy> TreeVec<T> for NormalizedTreeVector<T> {
    fn push(&mut self, value: T) -> i32 {
        let index = self.length;

        if index == self.max_length {
            self.allocate_level();
        }

        self.data.push(value);
        self.indexes.push(index as i32);
        self.length += 1;

        index as i32
    }

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        if index > self.length as i32 {
            None
        } else {
            let tree_index = TreeIndex {
                index,
                left_index: 2 * index + 1,
                right_index: 2 * index + 2,
                height: 0,
            };
            let data = self.data[index as usize];

            let node = TreeNode {
                value: data,
                indexes: tree_index,
            };

            Some(node)
        }
    }

    fn remove(&mut self, index: i32) -> Option<TreeNode<T>> {
        if index < 0 || index >= self.length as i32 {
            None
        } else {
            self.indexes[index as usize] = -1;
            let tree_index = TreeIndex {
                index,
                left_index: 2 * index + 1,
                right_index: 2 * index + 2,
                height: 0,
            };

            let node = TreeNode {
                value: self.data[index as usize],
                indexes: tree_index,
            };

            Some(node)
        }
    }

    fn len(&self) -> usize {
        self.length as usize
    }
}

impl <T: Default + Copy> Index<i32> for NormalizedTreeVector<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl <T: Default + Copy> IndexMut<i32> for NormalizedTreeVector<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

