use std::ops::{Index, IndexMut};
use crate::core::structs::tree::nodes::normalized_tree_index::NormalizedTreeIndex;
use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use crate::core::structs::tree::nodes::tree_node::TreeNode;
use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
use crate::core::structs::tree::vectors::tree_vec::{NormalizedTreeVecIndexes, OptimizedFunctions, TreeVec, TreeVecLevels};

pub struct NormalizedTreeVector<T> {
    allocated_levels: u8,
    max_length: u64,
    length: u64,

    data: Vec<T>,
    indexes: Vec<NormalizedTreeIndex>,
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

    pub fn swap_indexes(&mut self, index1: i32, index2: i32) {
        let index1_new = self.indexes[index1 as usize].index;
        let index2_new = self.indexes[index2 as usize].index;

        self.indexes[index1 as usize].index = index2_new;
        self.indexes[index2 as usize].index = index1_new;
    }

    pub fn get_parent_index(index: i32) -> i32 {
        if index == 0 {
            return -1;
        } else {
            let parent_index = (index - 1) / 2;
            parent_index as i32
        }
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
        let tree_index = NormalizedTreeIndex {
            index: index as i32,
            height: NormalizedTreeIndex::find_height(index as i32),
        };
        self.indexes.push(tree_index);
        self.length += 1;

        index as i32
    }

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        if index >= self.length as i32 || index < 0 {
            None
        } else {
            let tree_index = TreeIndex {
                index: self.indexes[index as usize].index,
                left_index: 2 * index + 1,
                right_index: 2 * index + 2,
                height: self.indexes[index as usize].height,
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
            let height = self.indexes[index as usize].height;
            let res_index = self.indexes[index as usize].index;
            let data = self.data[index as usize];

            if index == (self.length - 1) as i32 {
                self.data.pop();
                self.indexes.pop();
                self.length -= 1;
            } else {
                self.indexes[index as usize] = NormalizedTreeIndex::default();
            }
            let tree_index = TreeIndex {
                index: res_index,
                left_index: 2 * index + 1,
                right_index: 2 * index + 2,
                height,
            };

            let node = TreeNode {
                value: data,
                indexes: tree_index,
            };

            Some(node)
        }
    }

    fn len(&self) -> usize {
        self.length as usize
    }
}

impl <T: Default + Copy> NormalizedTreeVecIndexes<T> for NormalizedTreeVector<T> {
    fn get_value_mut(&mut self, index: i32) -> &mut T {
        &mut self.data[index as usize]
    }

    fn get_index_mut(&mut self, index: i32) -> &mut NormalizedTreeIndex {
        &mut self.indexes[index as usize]
    }

    fn get_index(&self, index: i32) -> &NormalizedTreeIndex {
        &self.indexes[index as usize]
    }

    fn get_indexes(&mut self) -> &mut Vec<NormalizedTreeIndex> {
        &mut self.indexes
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

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::vectors::normalized_tree_vec::NormalizedTreeVector;
    use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
    use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecLevels};

    #[test]
    fn test_normalized_tree_vector_new() {
        let vec = NormalizedTreeVector::<u64>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(vec.get_max_length(), 2u64.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_normalized_tree_vector_push() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);

        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(vec.get_max_length(), 2u64.pow(INITIAL_LEVELS as u32) - 1);
        assert_eq!(vec.indexes[0].height, 1);
        assert_eq!(vec.indexes[2].height, 2);
    }

    #[test]
    fn test_normalized_tree_vector_get() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        assert_eq!(vec.get(0).is_some(), true);
        assert_eq!(vec.get(0).unwrap().value, 1)
    }

    #[test]
    fn test_normalized_tree_vector_swap_indexes() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        vec.swap_indexes(0, 2);

        assert_eq!(vec.get(0).unwrap().indexes.index, 2);
        assert_eq!(vec.get(2).unwrap().indexes.index, 0);

        assert_eq!(vec.get(0).unwrap().indexes.height, 1);
        assert_eq!(vec.get(2).unwrap().indexes.height, 2);
    }

    #[test]
    fn test_normalized_tree_vector_get_parent() {
        assert_eq!(NormalizedTreeVector::<u64>::get_parent_index(0), -1);
        assert_eq!(NormalizedTreeVector::<u64>::get_parent_index(1), 0);
        assert_eq!(NormalizedTreeVector::<u64>::get_parent_index(5), 2);
    }
}

